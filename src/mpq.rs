#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
// include!("./stormbindings.rs");

use anyhow::*;
use std::ffi::CString;
use std::path::Path;

pub enum OpenFlags {
    None = 0,
    NoListFile = MPQ_OPEN_NO_LISTFILE as isize,
    NoAttributes = MPQ_OPEN_NO_ATTRIBUTES as isize,
    NoHeaderSearch = MPQ_OPEN_NO_HEADER_SEARCH as isize,
    ForceMPQ1 = MPQ_OPEN_FORCE_MPQ_V1 as isize,
    ForceListFile = MPQ_OPEN_FORCE_LISTFILE as isize,
    CHeckSectorCRC = MPQ_OPEN_CHECK_SECTOR_CRC as isize,
    Patch = MPQ_OPEN_PATCH as isize,
    ReadOnly = MPQ_OPEN_READ_ONLY as isize,
}

pub struct MPQ {
    handle: HANDLE,
}

impl MPQ {
    pub fn set_locale(locale: u32) -> u32 {
        unsafe { SFileSetLocale(locale) }
    }

    pub fn get_locale() -> u32 {
        unsafe { SFileGetLocale() }
    }

    pub fn version() -> String {
        unsafe {
            CString::from_vec_with_nul(STORMLIB_VERSION_STRING.to_vec())
                .unwrap()
                .into_string()
                .unwrap()
        }
    }

    pub fn create(filepath: &Path) -> Result<Self, anyhow::Error> {
        if !filepath.exists() {
            bail!("File not found {}", filepath.display())
        }

        let mut handle: HANDLE = std::ptr::null_mut();

        let path = filepath.to_str().unwrap();
        let szMpqName = CString::new(path).unwrap();
        let dwCreateFlags = 0;
        let dwMaxFileCount = 0;

        unsafe {
            let status = SFileCreateArchive(
                szMpqName.as_ptr(),
                dwCreateFlags,
                dwMaxFileCount,
                &mut handle,
            );

            let error_code = GetLastError();

            if error_code == 0 && status == 1 {
                return Self::open(filepath, None, None);
            }

            return Self::_parse_error::<Self>(error_code);
        }
    }

    pub fn open(
        filepath: &Path,
        priority: Option<u32>,
        flags: Option<OpenFlags>,
    ) -> Result<Self, anyhow::Error> {
        if !filepath.exists() {
            bail!("File not found {}", filepath.display())
        }

        let filepath = filepath.to_str().unwrap();
        let szMpqName = CString::new(filepath).unwrap();
        let mut handle: HANDLE = std::ptr::null_mut();

        unsafe {
            let status = SFileOpenArchive(
                szMpqName.as_ptr(),
                priority.unwrap_or(0),
                flags.unwrap_or(OpenFlags::None) as u32,
                &mut handle,
            );

            let error_code = GetLastError();

            if error_code == 0 && status == 1 {
                return Ok(Self { handle });
            }

            return Self::_parse_error::<Self>(error_code);
        }
    }

    pub fn close(&self) -> bool {
        unsafe {
            return SFileCloseArchive(self.handle) == 1;
        }
    }

    pub fn is_closed(&self) -> bool {
        unsafe { return self.handle.is_null() }
    }

    pub fn contains(&self, filename: String) -> bool {
        let filestring = CString::new(filename).unwrap();

        unsafe {
            let status = SFileHasFile(self.handle, filestring.as_ptr());

            return status == 1;
        }
    }

    pub fn extract(&self, file: String, target: String) -> bool {
        let filestr = CString::new(file).unwrap();
        let targetstr = CString::new(target).unwrap();

        unsafe {
            let status = SFileExtractFile(
                self.handle,
                filestr.as_ptr(),
                targetstr.as_ptr(),
                SFILE_OPEN_FROM_MPQ,
            );

            status == 1
        }
    }

    pub fn get_file(&self, file: String) -> Result<MPQFile, anyhow::Error> {
        let mut file_handle: HANDLE = std::ptr::null_mut();
        let filestr = CString::new(file).unwrap();

        unsafe {
            let status = SFileOpenFileEx(
                self.handle,
                filestr.as_ptr(),
                SFILE_OPEN_FROM_MPQ,
                &mut file_handle,
            );

            let error_code = GetLastError();

            if error_code == 0 && status == 1 {
                return Ok(MPQFile {
                    handle: file_handle,
                });
            }

            return Self::_parse_error::<MPQFile>(error_code);
        }
    }

    fn _parse_error<T>(status: u32) -> Result<T, anyhow::Error> {
        match status {
            ERROR_FILE_NOT_FOUND => {
                println!("File not found");
                return Err(anyhow!("File not found"));
            }
            ERROR_ACCESS_DENIED => {
                println!("Access denied");
                return Err(anyhow!("Access denied"));
            }
            ERROR_INVALID_HANDLE => {
                println!("Invalid handle");
                return Err(anyhow!("Invalid handle"));
            }
            ERROR_NOT_ENOUGH_MEMORY => {
                println!("Not enough memory");
                return Err(anyhow!("Not enough memory"));
            }
            ERROR_NOT_SUPPORTED => {
                println!("Not supported");
                return Err(anyhow!("Not supported"));
            }
            ERROR_INVALID_PARAMETER => {
                println!("Invalid parameter");
                return Err(anyhow!("Invalid parameter"));
            }
            ERROR_NEGATIVE_SEEK => {
                println!("Negative seek");
                return Err(anyhow!("Negative seek"));
            }
            ERROR_DISK_FULL => {
                println!("Disk full");
                return Err(anyhow!("Disk full"));
            }
            ERROR_ALREADY_EXISTS => {
                println!("Already exists");
                return Err(anyhow!("Already exists"));
            }
            ERROR_INSUFFICIENT_BUFFER => {
                println!("Insufficient buffer");
                return Err(anyhow!("Insufficient buffer"));
            }
            ERROR_BAD_FORMAT => {
                println!("Bad format");
                return Err(anyhow!("Bad format"));
            }
            ERROR_NO_MORE_FILES => {
                println!("No more files");
                return Err(anyhow!("No more files"));
            }
            ERROR_HANDLE_EOF => {
                println!("Handle EOF");
                return Err(anyhow!("Handle EOF"));
            }
            ERROR_CAN_NOT_COMPLETE => {
                println!("Can not complete");
                return Err(anyhow!("Can not complete"));
            }
            ERROR_FILE_CORRUPT => {
                println!("File corrupt");
                return Err(anyhow!("File corrupt"));
            }
            _ => {
                println!("Unknown error");
                return Err(anyhow!("Unknown error"));
            }
        }
    }
}

pub struct MPQFile {
    handle: HANDLE,
}

impl MPQFile {
    pub fn close(&self) -> bool {
        unsafe {
            return SFileCloseFile(self.handle) == 1;
        }
    }

    pub fn name(&self) -> String {
        let mut file_name_vec = [0i8; MAX_PATH as usize];
        let file_name_ptr = file_name_vec.as_mut_ptr();

        unsafe {
            let status = SFileGetFileName(self.handle, file_name_ptr);

            println!("file name {}", status);

            return CString::from_raw(file_name_ptr).into_string().unwrap();
        }
    }

    pub fn size(&self) -> u32 {
        let mut pdwFileSizeHigh: u32 = 0;
        let handle = self.handle;

        unsafe {
            let status = SFileGetFileSize(handle, &mut pdwFileSizeHigh);

            println!("file size {}", status);

            return pdwFileSizeHigh;
        }
    }
}

#[cfg(test)]
mod mpq_test {
    use super::*;

    fn w3m_fixture_path<'a>() -> &'a Path {
        Path::new("./tests/fixtures/TheDeathSheep.w3m")
    }

    #[test]
    fn test_open_archive() {
        let mpq = MPQ::open(w3m_fixture_path(), None, None);

        assert!(mpq.is_ok());
    }

    #[test]
    fn test_close_archive() {
        let mpq = MPQ::open(w3m_fixture_path(), None, None).unwrap();

        assert_eq!(mpq.close(), true);
    }

    #[test]
    fn test_get_file() {
        let mpq = MPQ::open(w3m_fixture_path(), None, None).unwrap();

        let file = mpq.get_file("(listfile)".to_string());

        assert!(file.is_ok());
    }

    #[test]
    fn test_get_file_size() {
        let mpq = MPQ::open(w3m_fixture_path(), None, None).ok().unwrap();

        let file = mpq.get_file("(listfile)".to_string()).ok().unwrap();

        assert_eq!(file.size(), 0);
    }

    #[test]
    fn test_version() {
        assert_eq!(MPQ::version(), "9.25");
    }

    #[test]
    fn test_locale() {
        assert_eq!(MPQ::get_locale(), 0);
    }
}
