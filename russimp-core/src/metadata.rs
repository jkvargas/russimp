use russimp_sys::aiMetadata;

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