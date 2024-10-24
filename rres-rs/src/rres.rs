use std::{
    ffi::{CStr, CString},
    ops::{Deref, DerefMut},
};

pub struct ResourceChunk(pub(crate) rres_sys::rresResourceChunk);
pub struct ResourceMulti(pub(crate) rres_sys::rresResourceMulti);
pub struct ResourceChunkInfo(pub(crate) rres_sys::rresResourceChunkInfo);
pub struct CentralDir(pub(crate) rres_sys::rresCentralDir);

impl ResourceChunk {
    pub fn new(file_name: &str, id: i32) -> Self {
        let r = unsafe {
            let cstr = CString::new(file_name).unwrap();
            rres_sys::rresLoadResourceChunk(cstr.as_ptr(), id)
        };
        Self(r)
    }
}

impl Drop for ResourceChunk {
    fn drop(&mut self) {
        unsafe { rres_sys::rresUnloadResourceChunk(self.0) }
    }
}

impl Deref for ResourceChunk {
    type Target = rres_sys::rresResourceChunk;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ResourceChunk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ResourceMulti {
    pub fn new(file_name: &str, id: i32) -> Self {
        let r = unsafe {
            let cstr = CString::new(file_name).unwrap();
            rres_sys::rresLoadResourceMulti(cstr.as_ptr(), id)
        };

        Self(r)
    }
}

impl Drop for ResourceMulti {
    fn drop(&mut self) {
        unsafe { rres_sys::rresUnloadResourceMulti(self.0) }
    }
}
impl Deref for ResourceMulti {
    type Target = rres_sys::rresResourceMulti;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ResourceMulti {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl ResourceChunkInfo {
    pub fn new(file_name: &str, id: i32) -> Self {
        let r = unsafe {
            let cstr = CString::new(file_name).unwrap();
            rres_sys::rresLoadResourceChunkInfo(cstr.as_ptr(), id)
        };
        Self(r)
    }
    pub fn all(file_name: &str) -> Vec<Self> {
        let mut i: u32 = 0;
        let r = unsafe {
            let cstr = CString::new(file_name).unwrap();
            rres_sys::rresLoadResourceChunkInfoAll(cstr.as_ptr(), &mut i)
        };
        unsafe {
            std::slice::from_raw_parts_mut(r, i as usize)
                .iter()
                .map(|f| Self(*f))
                .collect::<Vec<_>>()
        }
    }
}
impl Deref for ResourceChunkInfo {
    type Target = rres_sys::rresResourceChunkInfo;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ResourceChunkInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl CentralDir {
    pub fn new(file_name: &str) -> Self {
        let r = unsafe {
            let cstr = CString::new(file_name).unwrap();
            rres_sys::rresLoadCentralDirectory(cstr.as_ptr())
        };

        Self(r)
    }

    pub fn get_resource_id(&self, file_name: &str) -> i32 {
        unsafe {
            let cstr = CString::new(file_name).unwrap();
            rres_sys::rresGetResourceId(self.0, cstr.as_ptr())
        }
    }
}

impl Drop for CentralDir {
    fn drop(&mut self) {
        unsafe { rres_sys::rresUnloadCentralDirectory(self.0) }
    }
}
impl Deref for CentralDir {
    type Target = rres_sys::rresCentralDir;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for CentralDir {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
pub fn get_data_type(four_cc: [u8; 4]) -> u32 {
    unsafe { rres_sys::rresGetDataType(four_cc.as_ptr()) }
}

pub fn compute_crc32(data: &[u8]) -> u32 {
    unsafe { rres_sys::rresComputeCRC32(data.as_ptr(), data.len() as i32) }
}
pub fn set_cipher_password(pass: &str) {
    unsafe {
        let cstr = CString::new(pass).unwrap();
        rres_sys::rresSetCipherPassword(cstr.as_ptr())
    };
}

pub fn get_cipher_password<'a>() -> &'a str {
    unsafe {
        CStr::from_ptr(rres_sys::rresGetCipherPassword())
            .to_str()
            .unwrap()
    }
}
