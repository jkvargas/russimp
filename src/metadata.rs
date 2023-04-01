#![allow(non_upper_case_globals)]

use crate::{sys::*, *};
use derivative::Derivative;
use std::{ffi::CStr, os::raw::c_char};

trait MetaDataEntryCast {
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

impl<'a> MetaDataEntryCast for MetaDataEntryULong<'a> {
    fn cast(&self) -> Russult<MetadataType> {
        let raw = self.data.mData as *mut u64;

        if let Some(result) = unsafe { raw.as_ref() } {
            return Ok(MetadataType::ULong(*result));
        }

        Err(RussimpError::MetadataError(
            "Cant convert to ulong".to_string(),
        ))
    }
}

impl<'a> MetaDataEntryCast for MetaDataEntryInteger<'a> {
    fn cast(&self) -> Russult<MetadataType> {
        let raw = self.data.mData as *mut i32;

        if let Some(result) = unsafe { raw.as_ref() } {
            return Ok(MetadataType::Int(*result));
        }

        Err(RussimpError::MetadataError(
            "Cant convert to integer".to_string(),
        ))
    }
}

impl<'a> MetaDataEntryCast for MetaDataEntryBool<'a> {
    fn cast(&self) -> Russult<MetadataType> {
        let raw = self.data.mData as *mut bool;

        if let Some(result) = unsafe { raw.as_ref() } {
            return Ok(MetadataType::Bool(*result));
        }

        Err(RussimpError::MetadataError(
            "Cant convert to bool".to_string(),
        ))
    }
}

impl<'a> MetaDataEntryCast for MetaDataEntryDouble<'a> {
    fn cast(&self) -> Russult<MetadataType> {
        let raw = self.data.mData as *mut f64;

        if let Some(result) = unsafe { raw.as_ref() } {
            return Ok(MetadataType::Double(*result));
        }

        Err(RussimpError::MetadataError(
            "Cant convert to double".to_string(),
        ))
    }
}

impl<'a> MetaDataEntryCast for MetaDataEntryFloat<'a> {
    fn cast(&self) -> Russult<MetadataType> {
        let raw = self.data.mData as *mut f32;

        if let Some(result) = unsafe { raw.as_ref() } {
            return Ok(MetadataType::Float(*result));
        }

        Err(RussimpError::MetadataError(
            "Cant convert to float".to_string(),
        ))
    }
}

impl<'a> MetaDataEntryCast for MetaDataEntryString<'a> {
    fn cast(&self) -> Russult<MetadataType> {
        let raw = self.data.mData as *const aiString;
        
        if let Some(result) = unsafe { raw.as_ref() } {
            Ok(MetadataType::String(result.into()))
        }
        else
        {
            Err(RussimpError::MetadataError(
                "Cant convert to string".to_string(),
            ))
        }
    }
}

struct MetaDataVector3d<'a> {
    data: &'a aiMetadataEntry,
}

impl<'a> MetaDataEntryCast for MetaDataVector3d<'a> {
    fn cast(&self) -> Russult<MetadataType> {
        let vec: *const aiVector3D = self.data.mData as *const aiVector3D;
        if let Some(content) = unsafe { vec.as_ref() } {
            return Ok(MetadataType::Vector3d(content.into()));
        }

        Err(RussimpError::MetadataError("data is null".to_string()))
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct MetaData {
    pub keys: Vec<String>,
    pub values: Vec<MetaDataEntry>,
}

impl From<&aiMetadata> for MetaData {
    fn from(meta_data: &aiMetadata) -> Self {
        Self {
            keys: utils::get_vec(meta_data.mKeys, meta_data.mNumProperties),
            values: utils::get_vec(meta_data.mValues, meta_data.mNumProperties),
        }
    }
}

#[derive(Derivative, PartialEq)]
#[derivative(Debug)]
#[repr(u32)]
pub enum MetadataType {
    String(String),
    Vector3d(Vector3D),
    Bool(bool),
    Float(f32),
    Double(f64),
    Int(i32),
    ULong(u64),
    /* MetaMax = aiMetadataType_AI_META_MAX, -- Not sure what it does
     * Force32 = aiMetadataType_FORCE_32BIT, -- Not sure what it does */
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct MetaDataEntry(Russult<MetadataType>);

impl MetaDataEntry {
    fn cast_data(data: &aiMetadataEntry) -> Russult<MetadataType> {
        match data.mType {
            aiMetadataType_AI_AIVECTOR3D => MetaDataVector3d { data }.cast(),
            aiMetadataType_AI_AISTRING => MetaDataEntryString { data }.cast(),
            aiMetadataType_AI_BOOL => MetaDataEntryBool { data }.cast(),
            aiMetadataType_AI_FLOAT => MetaDataEntryFloat { data }.cast(),
            aiMetadataType_AI_DOUBLE => MetaDataEntryDouble { data }.cast(),
            aiMetadataType_AI_INT32 => MetaDataEntryInteger { data }.cast(),
            aiMetadataType_AI_UINT64 => MetaDataEntryULong { data }.cast(),
            _ => Err(RussimpError::MetadataError(
                "could not find caster for metadata type".to_string(),
            ))

        }
    }
}

impl From<&aiMetadataEntry> for MetaDataEntry {
    fn from(data: &aiMetadataEntry) -> Self {
        Self(Self::cast_data(data))
    }
}

#[test]
fn metadata_for_box() {
    use crate::scene::{PostProcess, Scene};

    let current_directory_buf = utils::get_model("models/BLEND/box.blend");

    let scene = Scene::from_file(
        current_directory_buf.as_str(),
        &[
            PostProcess::CalculateTangentSpace,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::SortByPrimitiveType,
        ],
    )
    .unwrap();

    let metadata = scene.metadata.unwrap();

    assert_eq!(1, metadata.keys.len());
    assert_eq!(1, metadata.values.len());

    assert_eq!("SourceAsset_Format".to_string(), metadata.keys[0]);
    assert_eq!((&metadata.values[0]).0.as_ref().unwrap(), &MetadataType::String("Blender 3D Importer (http://www.blender3d.org)".to_string()));
}

#[test]
fn debug_metadata() {
    use crate::scene::{PostProcess, Scene};

    let current_directory_buf = utils::get_model("models/BLEND/box.blend");

    let scene = Scene::from_file(
        current_directory_buf.as_str(),
        &[
            PostProcess::CalculateTangentSpace,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::SortByPrimitiveType,
        ],
    )
    .unwrap();

    dbg!(&scene.metadata);
}
