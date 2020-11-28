use russimp_sys::{aiMetadata,
                  aiMetadataType_AI_AISTRING,
                  aiMetadataType_AI_AIVECTOR3D,
                  aiMetadataType_AI_BOOL,
                  aiMetadataType_AI_FLOAT,
                  aiMetadataType_AI_DOUBLE,
                  aiMetadataType_AI_INT32,
                  aiMetadataType_AI_UINT64,
                  aiMetadataType_AI_META_MAX,
                  aiMetadataType_FORCE_32BIT,
                  aiMetadataEntry
};

use crate::{
    FromRaw,
    scene::{PostProcessSteps, Scene}
};

pub struct MetaData<'a> {
    meta_data: &'a aiMetadata,
    pub keys: Vec<String>,
    pub values: Vec<MetaDataEntry<'a>>,
}

pub struct MetaDataEntry<'a> {
    raw: &'a aiMetadataEntry
}

impl<'a> Into<MetaDataEntry<'a>> for &'a aiMetadataEntry {
    fn into(self) -> MetaDataEntry<'a> {
        MetaDataEntry {
            raw: self
        }
    }
}

#[derive(FromPrimitive, Debug, PartialEq)]
#[repr(u32)]
pub enum MetadataType {
    String = aiMetadataType_AI_AISTRING,
    Vector3d = aiMetadataType_AI_AIVECTOR3D,
    Bool = aiMetadataType_AI_BOOL,
    Float = aiMetadataType_AI_FLOAT,
    Double = aiMetadataType_AI_DOUBLE,
    Int = aiMetadataType_AI_INT32,
    Long = aiMetadataType_AI_UINT64,
    MetaMax = aiMetadataType_AI_META_MAX,
    Force32 = aiMetadataType_FORCE_32BIT,
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