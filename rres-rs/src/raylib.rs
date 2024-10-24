use std::ffi::{CStr, CString};

use crate::{ResourceChunk, ResourceMulti};
use raylib::{
    audio::{RaylibAudio, Wave},
    models::Mesh,
    text::Font,
    texture::Image,
    RaylibHandle,
};

pub trait RRESImpl {
    fn load_data_from_resource<'a>(&self, chunk: ResourceChunk) -> &'a [u8] {
        let mut i: u32 = 0;
        let d = unsafe { rres_sys::LoadDataFromResource(chunk.0, &mut i) };

        unsafe { std::slice::from_raw_parts_mut(d as *mut u8, i as usize) }
    }
    fn load_text_from_resource<'a>(&self, chunk: ResourceChunk) -> &'a str {
        let d = unsafe { rres_sys::LoadTextFromResource(chunk.0) };
        unsafe { CStr::from_ptr(d).to_str().unwrap() }
    }
    fn load_image_from_resource(&self, chunk: ResourceChunk) -> Image {
        let d = unsafe { rres_sys::LoadImageFromResource(chunk.0) };
        unsafe { Image::from_raw(d) }
    }
    fn load_font_from_resource(&self, multi: ResourceMulti) -> Font {
        let d = unsafe { rres_sys::LoadFontFromResource(multi.0) };
        unsafe { Font::from_raw(d) }
    }
    fn load_mesh_from_resource(&self, multi: ResourceMulti) -> Mesh {
        let d = unsafe { rres_sys::LoadMeshFromResource(multi.0) };
        unsafe { Mesh::from_raw(d) }
    }
    fn unpack_resource_chunk(&self, chunk: &mut [ResourceChunk]) -> i32 {
        unsafe {
            rres_sys::UnpackResourceChunk(
                chunk.iter().map(|f| f.0).collect::<Vec<_>>().as_mut_ptr(),
            )
        }
    }
    fn set_base_directory(&self, base_dir: &str) {
        unsafe {
            let s = CString::new(base_dir).unwrap();
            rres_sys::SetBaseDirectory(s.as_ptr())
        }
    }
}

impl<'a> RRESImpl for RaylibHandle {}

pub trait RRESAudio<'a>: AsRef<RaylibAudio> {
    fn load_wave_from_resource(&self, chunk: ResourceChunk) -> Wave<'a> {
        let raudio: &RaylibAudio = self.as_ref();

        unsafe { std::mem::transmute((rres_sys::LoadWaveFromResource(chunk.0), raudio)) }
    }
}
