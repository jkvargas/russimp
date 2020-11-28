use russimp_sys::aiFace;
use crate::FromRawVec;
use std::{
    os::raw::c_uint,
    ptr::slice_from_raw_parts
};

pub struct Face<'a> {
    face: &'a aiFace,
    pub indices: Vec<&'a u32>,
}

impl<'a> FromRawVec for Face<'a> {}

impl<'a> Into<Face<'a>> for &'a aiFace {
    fn into(self) -> Face<'a> {
        Face {
            face: self,
            indices: Face::get_rawvec(self.mIndices, self.mNumIndices)
        }
    }
}
