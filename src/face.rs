use crate::{
    Utils,
    sys::aiFace
};

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
