use crate::{FromRaw,
            scene::{PostProcessSteps, Scene},
            sys,
            Russult,
            RussimpError};

use std::{
    ffi::CStr,
    os::raw::c_char,
};

trait MetaDataEntryCast<'a> {
    fn can_cast(&self) -> bool;
    fn cast(&self) -> Russult<MetadataType<'a>>;
}

struct MetaDataEntryString<'a> {
    data: &'a aiMetadataEntry,
}

struct MetaDataEntryBool<'a> {
    data: &'a aiMetadataEntry,
}

struct MetaDataEntryFloat<'a> {
    data: &'a aiMetadataEntry,
}

struct MetaDataEntryDouble<'a> {
    data: &'a aiMetadataEntry,
}

struct MetaDataEntryInteger<'a> {
    data: &'a aiMetadataEntry,
}

struct MetaDataEntryULong<'a> {
    data: &'a aiMetadataEntry,
}

impl<'a> MetaDataEntryCast<'a> for MetaDataEntryULong<'a> {
    fn can_cast(&self) -> bool {
        (self.data.mType & aiMetadataType_AI_UINT64) != 0
    }

    fn cast(&self) -> Russult<MetadataType<'a>> {
        let raw = self.data.mData as *mut u64;

        if let Some(result) = unsafe { raw.as_ref() } {
            return Ok(MetadataType::ULong(result.clone()));
        }

        Err(RussimpError::MetadataError("Cant convert from bool".to_string()))
    }
}

impl<'a> MetaDataEntryCast<'a> for MetaDataEntryInteger<'a> {
    fn can_cast(&self) -> bool {
        (self.data.mType & aiMetadataType_AI_INT32) != 0
    }

    fn cast(&self) -> Russult<MetadataType<'a>> {
        let raw = self.data.mData as *mut i32;

        if let Some(result) = unsafe { raw.as_ref() } {
            return Ok(MetadataType::Int(result.clone()));
        }

        Err(RussimpError::MetadataError("Cant convert from bool".to_string()))
    }
}

impl<'a> MetaDataEntryCast<'a> for MetaDataEntryBool<'a> {
    fn can_cast(&self) -> bool {
        (self.data.mType & aiMetadataType_AI_BOOL) != 0
    }

    fn cast(&self) -> Russult<MetadataType<'a>> {
        let raw = self.data.mData as *mut bool;

        if let Some(result) = unsafe { raw.as_ref() } {
            return Ok(MetadataType::Bool(result.clone()));
        }

        Err(RussimpError::MetadataError("Cant convert from bool".to_string()))
    }
}

impl<'a> MetaDataEntryCast<'a> for MetaDataEntryDouble<'a> {
    fn can_cast(&self) -> bool {
        (self.data.mType & aiMetadataType_AI_DOUBLE) != 0
    }

    fn cast(&self) -> Russult<MetadataType<'a>> {
        let raw = self.data.mData as *mut f64;

        if let Some(result) = unsafe { raw.as_ref() } {
            return Ok(MetadataType::Double(result.clone()));
        }

        Err(RussimpError::MetadataError("Cant convert from bool".to_string()))
    }
}

impl<'a> MetaDataEntryCast<'a> for MetaDataEntryFloat<'a> {
    fn can_cast(&self) -> bool {
        (self.data.mType & aiMetadataType_AI_FLOAT) != 0
    }

    fn cast(&self) -> Russult<MetadataType<'a>> {
        let raw = self.data.mData as *mut f32;

        if let Some(result) = unsafe { raw.as_ref() } {
            return Ok(MetadataType::Float(result.clone()));
        }

        Err(RussimpError::MetadataError("Cant convert from bool".to_string()))
    }
}

impl<'a> MetaDataEntryCast<'a> for MetaDataEntryString<'a> {
    fn can_cast(&self) -> bool {
        (self.data.mType & aiMetadataType_AI_AISTRING) != 0
    }

    fn cast(&self) -> Russult<MetadataType<'a>> {
        let cstr = unsafe { CStr::from_ptr(self.data.mData as *const c_char) };
        cstr.to_str().map_or_else(|e| Err(e.into()), |r| Ok(MetadataType::String(r.to_string())))
    }
}

struct MetaDataVector3d<'a> {
    data: &'a aiMetadataEntry,
}

impl<'a> MetaDataEntryCast<'a> for MetaDataVector3d<'a> {
    fn can_cast(&self) -> bool {
        (self.data.mType & aiMetadataType_AI_AIVECTOR3D) != 0
    }

    fn cast(&self) -> Russult<MetadataType<'a>> {
        let vec = self.data.mData as *mut aiVector3D;
        if let Some(content) = unsafe { vec.as_ref() } {
            return Ok(MetadataType::Vector3d(content));
        }

        Err(RussimpError::MetadataError("data is null".to_string()))
    }
}

pub struct MetaData<'a> {
    meta_data: &'a aiMetadata,
    pub keys: Vec<String>,
    pub values: Vec<MetaDataEntry<'a>>,
}

#[repr(u32)]
pub enum MetadataType<'a> {
    String(String),
    Vector3d(&'a aiVector3D),
    Bool(bool),
    Float(f32),
    Double(f64),
    Int(i32),
    ULong(u64),
    // MetaMax = aiMetadataType_AI_META_MAX, -- Not sure what it does
    // Force32 = aiMetadataType_FORCE_32BIT, -- Not sure what it does
}

pub struct MetaDataEntry<'a> {
    raw: &'a aiMetadataEntry,
    pub data: Russult<MetadataType<'a>>,
}

impl<'a> MetaDataEntry<'a> {
    fn cast_data(data: &'a aiMetadataEntry) -> Russult<MetadataType<'a>> {
        let casters: Vec<Box<dyn MetaDataEntryCast<'a>>> = vec![
            Box::new(MetaDataVector3d {
                data
            }), Box::new(MetaDataEntryString {
                data
            }), Box::new(MetaDataEntryBool {
                data
            }), Box::new(MetaDataEntryFloat {
                data
            }), Box::new(MetaDataEntryDouble {
                data
            }), Box::new(MetaDataEntryInteger {
                data
            }), Box::new(MetaDataEntryULong {
                data
            })];

        for caster in casters {
            if caster.can_cast() {
                return caster.cast();
            }
        }

        Err(RussimpError::MetadataError("could not find caster for metadata type".to_string()))
    }
}

impl<'a> Into<MetaDataEntry<'a>> for &'a aiMetadataEntry {
    fn into(self) -> MetaDataEntry<'a> {
        MetaDataEntry {
            raw: self,
            data: MetaDataEntry::cast_data(self),
        }
    }
}

impl<'a> FromRaw for MetaData<'a> {}

impl<'a> Into<MetaData<'a>> for &'a aiMetadata {
    fn into(self) -> MetaData<'a> {
        MetaData {
            meta_data: self,
            keys: MetaData::get_vec(self.mKeys, self.mNumProperties),
            values: MetaData::get_vec(self.mValues, self.mNumProperties),
        }
    }
}

#[test]
fn metadata_for_box() {
    let current_directory_buf = std::env::current_dir().unwrap().join("models/BLEND/box.blend");

    let scene = Scene::from(current_directory_buf.to_str().unwrap(),
                            vec![PostProcessSteps::CalcTangentSpace,
                                 PostProcessSteps::Triangulate,
                                 PostProcessSteps::JoinIdenticalVertices,
                                 PostProcessSteps::SortByPType]).unwrap();

    assert!(scene.metadata.is_none());
}
