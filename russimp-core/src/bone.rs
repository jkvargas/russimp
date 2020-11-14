use russimp_sys::{
    aiBone,
    aiVertexWeight,
};

pub struct Bone {
    bone: *mut aiBone
}

impl Into<Bone> for *mut aiBone {
    fn into(self) -> Bone {
        Bone {
            bone: self
        }
    }
}

pub struct VertexWeight {
    vertex_weight: aiVertexWeight
}

impl Into<VertexWeight> for aiVertexWeight {
    fn into(self) -> VertexWeight {
        VertexWeight {
            vertex_weight: self
        }
    }
}

impl VertexWeight {
    fn get_vertex_id(&self) -> u32 {
        self.vertex_weight.mVertexId
    }

    fn get_weight(&self) -> f32 {
        self.vertex_weight.mWeight
    }
}

impl Bone {
    pub fn get_name(&self) -> String { unsafe { (*self.bone).mName }.into() }
}