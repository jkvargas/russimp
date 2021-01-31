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

impl Into<Face> for &aiFace {
    fn into(self) -> Face {
        Face {
            indices: Utils::get_rawvec(self.mIndices, self.mNumIndices)
        }
    }
}
