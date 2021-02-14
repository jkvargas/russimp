use crate::{sys::*, utils};
use derivative::Derivative;
use num_enum::TryFromPrimitive;
use num_traits::FromPrimitive;
use std::ptr::slice_from_raw_parts;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Material(Vec<MaterialProperty>);

impl From<&aiMaterial> for Material {
    fn from(material: &aiMaterial) -> Self {
        Material(utils::get_vec_from_raw(
            material.mProperties,
            material.mNumProperties,
        ))
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

#[derive(Derivative, FromPrimitive, PartialEq, TryFromPrimitive)]
#[derivative(Debug)]
#[repr(u32)]
pub enum TextureType {
    #[num_enum(default)]
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

impl From<&aiMaterialProperty> for MaterialProperty {
    fn from(material: &aiMaterialProperty) -> Self {
        let slice =
            slice_from_raw_parts(material.mData as *const u8, material.mDataLength as usize);
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
    use crate::scene::{PostProcess, Scene};

    let box_file_path = utils::get_model("models/BLEND/box.blend");

    let scene = Scene::from_file(
        box_file_path.as_str(),
        vec![
            PostProcess::CalculateTangentSpace,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::SortByPrimitiveType,
        ],
    )
    .unwrap();

    assert_eq!(1, scene.materials.len());
    assert_eq!(41, scene.materials[0].0.len());

    assert_eq!(false, scene.materials[0].0[40].data.is_empty());
    assert_eq!(
        "$mat.blend.mirror.glossAnisotropic",
        scene.materials[0].0[40].key.as_str()
    );
    assert_eq!(0, scene.materials[0].0[40].index);
    assert_eq!(
        PropertyTypeInfo::Float,
        scene.materials[0].0[40].material_type
    );
    assert_eq!(TextureType::None, scene.materials[0].0[40].semantic);
}

#[test]
fn debug_light() {
    use crate::scene::{PostProcess, Scene};

    let box_file_path = utils::get_model("models/BLEND/box.blend");

    let scene = Scene::from_file(
        box_file_path.as_str(),
        vec![
            PostProcess::CalculateTangentSpace,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::SortByPrimitiveType,
        ],
    )
    .unwrap();

    dbg!(&scene.lights);
}
