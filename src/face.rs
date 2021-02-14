use crate::{sys::aiFace, Utils};

use crate::scene::{PostProcessSteps, Scene};
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Face {
    pub indices: Vec<u32>,
}

impl Face {
    pub fn new(face: &aiFace) -> Face {
        Self {
            indices: Utils::get_rawvec(face.mIndices, face.mNumIndices),
        }
    }
}

#[test]
fn debug_face() {
    let current_directory_buf = Utils::get_model("models/3DS/CameraRollAnim.3ds");

    let scene = Scene::from(
        current_directory_buf.as_str(),
        vec![
            PostProcessSteps::CalculateTangentSpace,
            PostProcessSteps::Triangulate,
            PostProcessSteps::JoinIdenticalVertices,
            PostProcessSteps::SortByPrimitiveType,
        ],
    )
    .unwrap();

    dbg!(&scene.meshes[0].faces);
}
