use crate::{
    Utils,
    sys::aiFace
};

use derivative::Derivative;
use crate::scene::{Scene, PostProcessSteps};

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

    let scene = Scene::from(current_directory_buf.as_str(),
                            vec![PostProcessSteps::CalcTangentSpace,
                                 PostProcessSteps::Triangulate,
                                 PostProcessSteps::JoinIdenticalVertices,
                                 PostProcessSteps::SortByPType]).unwrap();

    dbg!(&scene.meshes[0].faces);
}