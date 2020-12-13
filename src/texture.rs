use std::ffi::CStr;

use crate::{
    FromRaw,
    sys::{aiTexture, aiTexel},
    scene::{Scene, PostProcessSteps}
};

pub struct Texture<'a> {
    texture: &'a aiTexture,
    filename: String,
    height: u32,
    width: u32,
    ach_format_hint: String,
    data: Vec<&'a aiTexel>,
}

impl<'a> FromRaw for Texture<'a> {}

impl<'a> Into<Texture<'a>> for &'a aiTexture {
    fn into(self) -> Texture<'a> {
        let content = unsafe { CStr::from_ptr(self.achFormatHint.as_ptr()) };
        let ach_format_hint = content.to_str().unwrap().to_string();

        Texture {
            texture: self,
            filename: self.mFilename.into(),
            height: self.mHeight,
            width: self.mWidth,
            ach_format_hint,
            data: Texture::get_rawvec(self.pcData, self.mHeight * self.mWidth),
        }
    }
}