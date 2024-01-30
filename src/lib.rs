#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
extern crate libc;

use napi::*;
use mpq::{ MPQ, MPQFile };
use std::path::Path;

mod mpq;

#[napi(js_name = "MpqArchive")]
struct JsMpqArchive {
    mpq: Option<MPQ>
}

#[napi]
impl JsMpqArchive {
    #[napi(factory)]
    pub fn open(filepath: String) -> Self {
        JsMpqArchive { mpq : MPQ::open(Path::new(&filepath), None, None).ok() }
    }

    #[napi(constructor)]
    pub fn new(filepath: String) -> Self {
        JsMpqArchive { mpq : MPQ::open(Path::new(&filepath), None, None).ok() }
    }

    #[napi]
    pub fn get_file(&mut self, name: String) -> JsMpqFile {
        let file = self.mpq.as_mut().unwrap().get_file(name).ok();

        JsMpqFile {
            file: file
        }
    }
}

#[napi(js_name = "MpqFile")]
struct JsMpqFile {
    file: Option<MPQFile>
}

#[napi]
impl JsMpqFile {
    // dummy method to proper ts types generation
    #[napi(constructor)]
    pub fn new() -> Self {
        Self { file: None }
    }

    #[napi(getter)]
    pub fn size(&mut self) -> napi::Result<u32> {
        Ok(self.file.as_mut().unwrap().size())
    }

    #[napi(getter)]
    pub fn name(&mut self) -> napi::Result<String> {
        Ok(self.file.as_mut().unwrap().name())
    }
}
