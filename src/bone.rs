use crate::{
    sys::{aiBone, aiVertexWeight},
    *,
};
use derivative::Derivative;

#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct Bone {
    pub weights: Vec<VertexWeight>,
    pub name: String,
    pub offset_matrix: Matrix4x4,
}

impl From<&aiBone> for Bone {
    fn from(bone: &aiBone) -> Self {
        Bone {
            weights: utils::get_vec(bone.mWeights, bone.mNumWeights),
            name: bone.mName.into(),
            offset_matrix: (&bone.mOffsetMatrix).into(),
        }
    }
}

#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct VertexWeight {
    pub weight: f32,
    pub vertex_id: u32,
}

impl From<&aiVertexWeight> for VertexWeight {
    fn from(vertex: &aiVertexWeight) -> Self {
        Self {
            weight: vertex.mWeight,
            vertex_id: vertex.mVertexId,
        }
    }
}

#[test]
fn debug_bones() {
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

    dbg!(&scene.meshes[0].bones);
}
