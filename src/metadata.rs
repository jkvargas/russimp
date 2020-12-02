use russimp_sys::{aiMetadata, aiMetadataType_AI_AISTRING, aiMetadataType_AI_AIVECTOR3D, aiMetadataType_AI_BOOL, aiMetadataType_AI_FLOAT, aiMetadataType_AI_DOUBLE, aiMetadataType_AI_INT32, aiMetadataType_AI_UINT64, aiMetadataType_AI_META_MAX, aiMetadataType_FORCE_32BIT, aiMetadataEntry, aiVector3D};

use crate::{FromRaw,
            scene::{PostProcessSteps, Scene},
            Russult,
            RussimpError};

use std::{
    any::Any,
    ffi::CStr,
    os::raw::c_char,
    borrow::Borrow
};

pub trait MetaDataEntryCast<'a> {
    fn can_cast(metadata_entry: &'a aiMetadataEntry) -> bool;
    fn cast(&mut self, metadata_entry: &'a aiMetadataEntry) -> Russult<&'a dyn Any>;
}

#[derive(Default)]
struct MetaDataEntryString {
    result: String
}

impl<'a> MetaDataEntryCast<'a> for MetaDataEntryString {
    fn can_cast(metadata_entry: &'a aiMetadataEntry) -> bool {
        (metadata_entry.mType & aiMetadataType_AI_AISTRING) != 0
    }

    fn cast(&mut self, metadata_entry: &'a aiMetadataEntry) -> Russult<&'a dyn Any> {
        let cstr = unsafe { CStr::from_ptr(metadata_entry.mData as *const c_char) };
        cstr.to_str().map_or_else(|e| Err(e.into()), |r| {
            self.result = r.to_string();
            Ok(&self.result)
        })
    }
}

#[derive(Default)]
struct MetaDataVector3d<'a> {
    result: &'a aiVector3D
}

impl<'a> MetaDataEntryCast<'a> for MetaDataVector3d<'a> {
    fn can_cast(metadata_entry: &'a aiMetadataEntry) -> bool {
        (metadata_entry.mType & aiMetadataType_AI_AIVECTOR3D) != 0
    }

    fn cast(&mut self, metadata_entry: &'a aiMetadataEntry) -> Russult<&'a dyn Any> {
        self.result = unsafe { (metadata_entry.mData as *const aiVector3D).as_ref() }.unwrap();
        Ok(self.result)
    }
}

pub struct MetaData<'a> {
    meta_data: &'a aiMetadata,
    pub keys: Vec<String>,
    pub values: Vec<MetaDataEntry<'a>>,
}

pub struct MetaDataEntry<'a> {
    raw: &'a aiMetadataEntry,
    pub data: Russult<&'a dyn Any>,
}

impl<'a> MetaDataEntry<'a> {
    fn cast_data(data: &'a aiMetadataEntry) -> Russult<&'a dyn Any> {
        let mut casters: Vec<Box<dyn MetaDataEntryCast>> = vec![Box::new(MetaDataEntryString::default())];

        for caster in &mut casters {
            if caster.can_cast(data) {
                return caster.cast(data);
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
    let current_directory_buf = std::env::current_dir().unwrap().join("russimp-sys/assimp/test/models/BLEND/box.blend");

    let scene = Scene::from(current_directory_buf.to_str().unwrap(),
                            vec![PostProcessSteps::CalcTangentSpace,
                                 PostProcessSteps::Triangulate,
                                 PostProcessSteps::JoinIdenticalVertices,
                                 PostProcessSteps::SortByPType]).unwrap();

    assert!(scene.metadata.is_none());
}