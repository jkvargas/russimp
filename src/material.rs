use std::{
    ptr::slice_from_raw_parts,
};

use crate::{
    sys::{
        aiMaterial,
        aiMaterialProperty,
        aiPropertyTypeInfo_aiPTI_Float,
        aiPropertyTypeInfo_aiPTI_Double,
        aiPropertyTypeInfo_aiPTI_Buffer,
        aiPropertyTypeInfo__aiPTI_Force32Bit,
        aiPropertyTypeInfo_aiPTI_Integer,
        aiPropertyTypeInfo_aiPTI_String,
        aiTextureType_aiTextureType_NONE,
        aiTextureType_aiTextureType_DIFFUSE,
        aiTextureType_aiTextureType_SPECULAR,
        aiTextureType_aiTextureType_AMBIENT,
        aiTextureType_aiTextureType_EMISSIVE,
        aiTextureType_aiTextureType_HEIGHT,
        aiTextureType_aiTextureType_NORMALS,
        aiTextureType_aiTextureType_SHININESS,
        aiTextureType_aiTextureType_OPACITY,
        aiTextureType_aiTextureType_DISPLACEMENT,
        aiTextureType_aiTextureType_LIGHTMAP,
        aiTextureType_aiTextureType_REFLECTION,
        aiTextureType_aiTextureType_BASE_COLOR,
        aiTextureType_aiTextureType_NORMAL_CAMERA,
        aiTextureType_aiTextureType_EMISSION_COLOR,
        aiTextureType_aiTextureType_METALNESS,
        aiTextureType_aiTextureType_DIFFUSE_ROUGHNESS,
        aiTextureType_aiTextureType_AMBIENT_OCCLUSION,
        aiTextureType_aiTextureType_UNKNOWN,
        aiTextureType__aiTextureType_Force32Bit,
    },
    scene::{PostProcessSteps,
            Scene
    },
    Utils
};

use derivative::Derivative;
use num_traits::FromPrimitive;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Material {
    properties: Vec<MaterialProperty>,
}

impl Material {
    pub fn new(material: &aiMaterial) -> Material {
        Material {
            properties: Utils::get_vec_from_raw(material.mProperties, material.mNumProperties, &MaterialProperty::new),
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct MaterialProperty {
    key: String,
    data: Vec<u8>,
    index: usize,
    material_type: PropertyTypeInfo,
    semantic: TextureType,
}

#[derive(Derivative, FromPrimitive, PartialEq)]
#[derivative(Debug)]
#[repr(u32)]
pub enum PropertyTypeInfo {
    Force32Bit = aiPropertyTypeInfo__aiPTI_Force32Bit,
    Buffer = aiPropertyTypeInfo_aiPTI_Buffer,
    Double = aiPropertyTypeInfo_aiPTI_Double,
    Float = aiPropertyTypeInfo_aiPTI_Float,
    Integer = aiPropertyTypeInfo_aiPTI_Integer,
    String = aiPropertyTypeInfo_aiPTI_String,
}

#[derive(Derivative, FromPrimitive, PartialEq)]
#[derivative(Debug)]
#[repr(u32)]
pub enum TextureType {
    None = aiTextureType_aiTextureType_NONE,
    Diffuse = aiTextureType_aiTextureType_DIFFUSE,
    Specular = aiTextureType_aiTextureType_SPECULAR,
    Ambient = aiTextureType_aiTextureType_AMBIENT,
    Emissive = aiTextureType_aiTextureType_EMISSIVE,
    Height = aiTextureType_aiTextureType_HEIGHT,
    Normals = aiTextureType_aiTextureType_NORMALS,
    Shininess = aiTextureType_aiTextureType_SHININESS,
    Opacity = aiTextureType_aiTextureType_OPACITY,
    Displacement = aiTextureType_aiTextureType_DISPLACEMENT,
    LightMap = aiTextureType_aiTextureType_LIGHTMAP,
    Reflection = aiTextureType_aiTextureType_REFLECTION,
    BaseColor = aiTextureType_aiTextureType_BASE_COLOR,
    NormalCamera = aiTextureType_aiTextureType_NORMAL_CAMERA,
    EmissionColor = aiTextureType_aiTextureType_EMISSION_COLOR,
    Metalness = aiTextureType_aiTextureType_METALNESS,
    Roughness = aiTextureType_aiTextureType_DIFFUSE_ROUGHNESS,
    AmbientOcclusion = aiTextureType_aiTextureType_AMBIENT_OCCLUSION,
    Unknown = aiTextureType_aiTextureType_UNKNOWN,
    Force32bit = aiTextureType__aiTextureType_Force32Bit,
}

impl MaterialProperty {
    pub fn new(material: &aiMaterialProperty) -> MaterialProperty {
        let slice = slice_from_raw_parts(material.mData as *const u8, material.mDataLength as usize);
        let data = unsafe { slice.as_ref() }.unwrap();

        MaterialProperty {
            key: material.mKey.into(),
            data: data.to_vec(),
            index: material.mIndex as usize,
            material_type: FromPrimitive::from_u32(material.mType as u32).unwrap(),
            semantic: FromPrimitive::from_u32(material.mSemantic as u32).unwrap(),
        }
    }
}

#[test]
fn material_for_box() {
    let box_file_path = get_model("models/BLEND/box.blend");

    let scene = Scene::from(box_file_path.as_str(),
                            vec![PostProcessSteps::CalcTangentSpace,
                                 PostProcessSteps::Triangulate,
                                 PostProcessSteps::JoinIdenticalVertices,
                                 PostProcessSteps::SortByPType]).unwrap();

    assert_eq!(1, scene.materials.len());
    assert_eq!(41, scene.materials[0].properties.len());

    assert_eq!(false, scene.materials[0].properties[40].data.is_empty());
    assert_eq!("$mat.blend.mirror.glossAnisotropic", scene.materials[0].properties[40].key.as_str());
    assert_eq!(0, scene.materials[0].properties[40].index);
    assert_eq!(PropertyTypeInfo::Float, scene.materials[0].properties[40].material_type);
    assert_eq!(TextureType::None, scene.materials[0].properties[40].semantic);
}