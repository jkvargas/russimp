use russimp_sys::{aiBone, aiVertexWeight, aiMatrix4x4};
use crate::FromRawVec;

pub struct Bone<'a> {
    bone: &'a aiBone,
    pub weights: Vec<VertexWeight<'a>>,
    pub name: String,
    pub offset_matrix: aiMatrix4x4
}

impl FromRawVec for Bone {}

impl<'a> Into<Bone> for &'a aiBone {
    fn into(self) -> Bone {
        Bone {
            bone: self,
            weights: Bone::get_vec(self.mWeights, self.mNumWeights),
            name: self.mName.into(),
            offset_matrix: self.mOffsetMatrix
        }
    }
}

pub struct VertexWeight<'a> {
    vertex_weight: &'a aiVertexWeight,
    pub weight: f32,
    pub vertex_id: u32
}

impl<'a> Into<VertexWeight> for &'a aiVertexWeight {
    fn into(self) -> VertexWeight {
        VertexWeight {
            vertex_weight: self,
            vertex_id: self.mVertexId,
            weight: self.mWeight
        }
    }
}