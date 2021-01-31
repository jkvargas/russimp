use crate::{
    Utils,
    sys::{
        aiBone,
        aiVertexWeight,
        aiMatrix4x4,
    }
};

use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Bone {
    pub weights: Vec<VertexWeight>,
    pub name: String,
    pub offset_matrix: aiMatrix4x4,
}

impl Into<Bone> for &aiBone {
    fn into(self) -> Bone {
        Bone {
            weights: Utils::get_vec(self.mWeights, self.mNumWeights, &VertexWeight::convert),
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

impl VertexWeight {
    pub fn new(vertex_id: u32, weight: f32) -> VertexWeight {
        VertexWeight {
            vertex_id,
            weight,
        }
    }

    pub fn convert(vertex: &aiVertexWeight) -> VertexWeight {
        VertexWeight::new(vertex.mVertexId, vertex.mWeight)
    }
}

// impl Into<VertexWeight> for &aiVertexWeight {
//     fn into(self) -> VertexWeight {
//         VertexWeight {
//             vertex_id: self.mVertexId,
//             weight: self.mWeight,
//         }
//     }
// }