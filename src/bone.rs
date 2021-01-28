use crate::{
    FromRaw,
    sys::{
        aiBone,
        aiVertexWeight,
        aiMatrix4x4,
    },
};

use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Bone {
    pub weights: Vec<VertexWeight>,
    pub name: String,
    pub offset_matrix: aiMatrix4x4,
}

impl FromRaw for Bone {}

impl Into<Bone> for &aiBone {
    fn into(self) -> Bone {
        Bone {
            weights: Bone::get_vec(self.mWeights, self.mNumWeights),
            name: self.mName.into(),
            offset_matrix: self.mOffsetMatrix,
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct VertexWeight {
    pub weight: f32,
    pub vertex_id: u32,
}

impl Into<VertexWeight> for &aiVertexWeight {
    fn into(self) -> VertexWeight {
        VertexWeight {
            vertex_id: self.mVertexId,
            weight: self.mWeight,
        }
    }
}