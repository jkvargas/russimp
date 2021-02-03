use crate::{Utils, sys::{
    aiBone,
    aiVertexWeight,
    aiMatrix4x4,
}, Matrix4x4};

use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Bone {
    pub weights: Vec<VertexWeight>,
    pub name: String,
    pub offset_matrix: Matrix4x4,
}

impl Bone {
    pub fn new(bone: &aiBone) -> Bone {
        Bone {
            weights: Utils::get_vec(bone.mWeights, bone.mNumWeights, &VertexWeight::convert),
            name: bone.mName.into(),
            offset_matrix: Matrix4x4::new(&bone.mOffsetMatrix),
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct VertexWeight {
    pub weight: f32,
    pub vertex_id: u32,
}

impl VertexWeight {
    pub fn new(vertex_id: u32, weight: f32) -> VertexWeight {
        VertexWeight {
            weight,
            vertex_id
        }
    }

    pub fn convert(vertex: &aiVertexWeight) -> VertexWeight {
        VertexWeight::new(vertex.mVertexId, vertex.mWeight)
    }
}