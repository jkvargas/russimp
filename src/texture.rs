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

impl Texture {
    pub fn new(texture: &aiTexture) -> Texture {
        let content = unsafe { CStr::from_ptr(texture.achFormatHint.as_ptr()) };
        let ach_format_hint = content.to_str().unwrap().to_string();

        Texture {
            filename: texture.mFilename.into(),
            height: texture.mHeight,
            width: texture.mWidth,
            ach_format_hint,
            data: Utils::get_rawvec(texture.pcData, texture.mHeight * texture.mWidth),
        }
    }
}