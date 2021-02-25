use crate::{
    sys::{aiTexel, aiTexture},
    *,
};
use derivative::Derivative;
use std::ffi::CStr;

#[repr(C, packed)]
#[derive(Derivative, Copy, Clone)]
#[derivative(Debug)]
pub struct Texel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

impl From<&aiTexel> for Texel {
    fn from(texel: &aiTexel) -> Self {
        Texel {
            b: texel.b,
            g: texel.g,
            r: texel.r,
            a: texel.a,
        }
    }
}

#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct Texture {
    pub filename: String,
    pub height: u32,
    pub width: u32,
    pub ach_format_hint: String,
    pub data: Vec<Texel>,
}

impl From<&aiTexture> for Texture {
    fn from(texture: &aiTexture) -> Self {
        let content = unsafe { CStr::from_ptr(texture.achFormatHint.as_ptr()) };
        let ach_format_hint = content.to_str().unwrap().to_string();

        Texture {
            filename: texture.mFilename.into(),
            height: texture.mHeight,
            width: texture.mWidth,
            ach_format_hint,
            data: utils::get_vec(texture.pcData, texture.mHeight * texture.mWidth),
        }
    }
}

#[test]
fn debug_texture() {
    use crate::scene::{PostProcess, Scene};

    let current_directory_buf = utils::get_model("models/BLEND/box.blend");

    let scene = Scene::from_file(
        current_directory_buf.as_str(),
        vec![
            PostProcess::CalculateTangentSpace,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::SortByPrimitiveType,
        ],
    )
    .unwrap();

    dbg!(&scene.textures);
}
