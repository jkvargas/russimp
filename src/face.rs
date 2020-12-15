use crate::{
    FromRaw,
    sys::aiFace
};

use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Face<'a> {
    #[derivative(Debug = "ignore")]
    face: &'a aiFace,
    pub indices: Vec<&'a u32>,
}

impl<'a> FromRaw for Face<'a> {}

impl<'a> Into<Face<'a>> for &'a aiFace {
    fn into(self) -> Face<'a> {
        Face {
            face: self,
            indices: Face::get_rawvec(self.mIndices, self.mNumIndices)
        }
    }
}
