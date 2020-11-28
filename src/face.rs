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
        let raw = slice_from_raw_parts(self.mIndices as *const c_uint, self.mNumIndices as usize);

        Face {
            face: self,
            indices: unsafe { raw.as_ref().unwrap() }.iter().collect()
        }
    }
}
