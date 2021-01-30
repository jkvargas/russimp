use crate::{
    FromRaw,
    scene::{
        PostProcessSteps,
        Scene,
    },
    sys::{
        aiMetadataEntry,
        aiMetadataType_AI_UINT64,
        aiMetadataType_AI_INT32,
        aiMetadataType_AI_BOOL,
        aiMetadataType_AI_DOUBLE,
        aiMetadataType_AI_FLOAT,
        aiMetadataType_AI_AISTRING,
        aiMetadataType_AI_AIVECTOR3D,
        aiVector3D,
        aiMetadata,
    },
    Russult,
    RussimpError,
    get_model,
};

use std::{
    ffi::CStr,
    os::raw::c_char,
};

use derivative::Derivative;

trait MetaDataEntryCast<'a> {
    fn can_cast(&self) -> bool;
    fn cast(&self) -> Russult<MetadataType>;
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

    fn cast(&self) -> Russult<MetadataType> {
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

    fn cast(&self) -> Russult<MetadataType> {
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

    fn cast(&self) -> Russult<MetadataType> {
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

    fn cast(&self) -> Russult<MetadataType> {
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

    fn cast(&self) -> Russult<MetadataType> {
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

    fn cast(&self) -> Russult<MetadataType> {
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

    fn cast(&self) -> Russult<MetadataType> {
        let vec = self.data.mData as *mut aiVector3D;
        if let Some(content) = unsafe { vec.as_ref() } {
            return Ok(MetadataType::Vector3d(content.clone()));
        }

        Err(RussimpError::MetadataError("data is null".to_string()))
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct MetaData {
    #[derivative(Debug = "ignore")]
    pub keys: Vec<String>,
    pub values: Vec<MetaDataEntry>,
}

#[derive(Derivative)]
#[derivative(Debug)]
#[repr(u32)]
pub enum MetadataType {
    String(String),
    Vector3d(aiVector3D),
    Bool(bool),
    Float(f32),
    Double(f64),
    Int(i32),
    ULong(u64),
    // MetaMax = aiMetadataType_AI_META_MAX, -- Not sure what it does
    // Force32 = aiMetadataType_FORCE_32BIT, -- Not sure what it does
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct MetaDataEntry {
    pub data: Russult<MetadataType>,
}

impl MetaDataEntry {
    fn cast_data(data: &aiMetadataEntry) -> Russult<MetadataType> {
        let casters: Vec<Box<dyn MetaDataEntryCast>> = vec![
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

impl Into<MetaDataEntry> for &aiMetadataEntry {
    fn into(self) -> MetaDataEntry {
        MetaDataEntry {
            data: MetaDataEntry::cast_data(self),
        }
    }
}

impl FromRaw for MetaData {}

impl Into<MetaData> for &aiMetadata {
    fn into(self) -> MetaData {
        MetaData {
            keys: MetaData::get_vec(self.mKeys, self.mNumProperties),
            values: MetaData::get_vec(self.mValues, self.mNumProperties),
        }
    }
}

#[test]
fn metadata_for_box() {
    let current_directory_buf = get_model("models/BLEND/box.blend");

    let scene = Scene::from(current_directory_buf.as_str(),
                            vec![PostProcessSteps::CalcTangentSpace,
                                 PostProcessSteps::Triangulate,
                                 PostProcessSteps::JoinIdenticalVertices,
                                 PostProcessSteps::SortByPType]).unwrap();

    assert!(scene.metadata.is_none());
}
