use russimp_sys::aiFace;
use crate::FromRaw;

pub struct Face<'a> {
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
