use russimp_sys::{aiBone, aiVertexWeight, aiMatrix4x4};
use crate::FromRaw;

pub struct Bone<'a> {
    bone: &'a aiBone,
    pub weights: Vec<VertexWeight<'a>>,
    pub name: String,
    pub offset_matrix: aiMatrix4x4
}

impl<'a> FromRaw for Bone<'a> {}

impl<'a> Into<Bone<'a>> for &'a aiBone {
    fn into(self) -> Bone<'a> {
        Bone {
            bone: self,
            weights: Bone::get_vec(self.mWeights, self.mNumWeights),
            name: self.mName.into(),
            offset_matrix: self.mOffsetMatrix
        }
    }
}

#[derive(Debug)]
pub struct VertexWeight<'a> {
    // #[derivative(Debug="ignore")]
    vertex_weight: &'a aiVertexWeight,
    pub weight: f32,
    pub vertex_id: u32
}

impl<'a> Into<VertexWeight<'a>> for &'a aiVertexWeight {
    fn into(self) -> VertexWeight<'a> {
        VertexWeight {
            vertex_weight: self,
            vertex_id: self.mVertexId,
            weight: self.mWeight
        }
    }
}