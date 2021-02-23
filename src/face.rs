use crate::{sys::aiFace, *};
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Face(pub Vec<u32>);

impl From<&aiFace> for Face {
    fn from(face: &aiFace) -> Self {
        Self(utils::get_raw_vec(face.mIndices, face.mNumIndices))
    }
}

#[test]
fn debug_face() {
    use crate::scene::{PostProcess, Scene};

    let current_directory_buf = utils::get_model("models/3DS/CameraRollAnim.3ds");

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

    dbg!(&scene.meshes[0].faces);
}
