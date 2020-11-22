use russimp_sys::aiMetadata;
use std::borrow::Borrow;

pub struct MetaData {
    meta_data: *mut aiMetadata
}

impl Into<MetaData> for *mut aiMetadata {
    fn into(self) -> MetaData {
        MetaData {
            meta_data: self
        }
    }
}

impl MetaData {
    pub fn get_keys(&self) -> Vec<String> {
        let res = unsafe { std::slice::from_raw_parts_mut((*self.meta_data).mKeys, (*self.meta_data).mNumProperties as usize) };
        res.to_vec().into_iter().map(|x|x.into()).collect()
    }
}