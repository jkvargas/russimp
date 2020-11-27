use russimp_sys::aiFace;
use crate::FromRawVec;

pub struct Face<'a> {
    face: &'a aiFace,
    pub indices: Vec<u32>,
}

impl FromRawVec for Face {}

impl<'a> Into<Face> for &'a aiFace {
    fn into(self) -> Face {
        Face {
            face: self,
            indices: Face::get_vec(self.mIndices, self.mNumIndices)
        }
    }
}
