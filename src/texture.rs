use std::ffi::CStr;

use crate::{sys::{
    aiTexture,
    aiTexel,
}, scene::{
    Scene,
    PostProcessSteps,
}, Utils};

use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Texture {
    filename: String,
    height: u32,
    width: u32,
    ach_format_hint: String,
    data: Vec<aiTexel>,
}

impl Into<Texture> for &aiTexture {
    fn into(self) -> Texture {
        let content = unsafe { CStr::from_ptr(self.achFormatHint.as_ptr()) };
        let ach_format_hint = content.to_str().unwrap().to_string();

        Texture {
            filename: self.mFilename.into(),
            height: self.mHeight,
            width: self.mWidth,
            ach_format_hint,
            data: Utils::get_rawvec(self.pcData, self.mHeight * self.mWidth),
        }
    }
}