use crate::{
    FromRaw,
    sys::aiFace
};

use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Face {
    pub indices: Vec<u32>,
}

impl FromRaw for Face {}

impl Into<Face> for &aiFace {
    fn into(self) -> Face {
        Face {
            indices: Face::get_rawvec(self.mIndices, self.mNumIndices)
        }
    }
}
