use std::ffi::CStr;

use crate::{
    scene::{PostProcessSteps, Scene},
    sys::{aiTexel, aiTexture},
    Utils,
};

use derivative::Derivative;

#[repr(C, packed)]
#[derive(Derivative, Copy, Clone)]
#[derivative(Debug)]
pub struct Texel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

impl Texel {
    fn new(texel: &aiTexel) -> Texel {
        Texel {
            b: texel.b,
            g: texel.g,
            r: texel.r,
            a: texel.a,
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Texture {
    filename: String,
    height: u32,
    width: u32,
    ach_format_hint: String,
    data: Vec<Texel>,
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
            data: Utils::get_vec(
                texture.pcData,
                texture.mHeight * texture.mWidth,
                &Texel::new,
            ),
        }
    }
}

#[test]
fn debug_texture() {
    let current_directory_buf = Utils::get_model("models/BLEND/box.blend");

    let scene = Scene::from(
        current_directory_buf.as_str(),
        vec![
            PostProcessSteps::CalcTangentSpace,
            PostProcessSteps::Triangulate,
            PostProcessSteps::JoinIdenticalVertices,
            PostProcessSteps::SortByPType,
        ],
    )
    .unwrap();

    dbg!(&scene.textures);
}
