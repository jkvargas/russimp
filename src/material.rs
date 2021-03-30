use crate::{sys::*, utils, RussimpError, Russult};
use derivative::Derivative;
use num_enum::TryFromPrimitive;
use num_traits::FromPrimitive;
use std::{mem::MaybeUninit, ptr::slice_from_raw_parts};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Material(pub Vec<MaterialProperty>);

impl Material {
    fn get_properties(material: &aiMaterial) -> Vec<MaterialProperty> {
        let properties =
            slice_from_raw_parts(material.mProperties, material.mNumProperties as usize);
        if properties.is_null() {
            return vec![];
        }

        let raw = unsafe { properties.as_ref() }.unwrap();
        let mut result = Vec::new();

        for item in raw {
            let property = unsafe { item.as_ref() }.unwrap();
            result.push(MaterialProperty::new(material, property));
        }

        result
    }
}

impl From<&aiMaterial> for Material {
    fn from(material: &aiMaterial) -> Self {
        Material(Self::get_properties(material))
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct MaterialProperty {
    //
    pub key: String,
    pub data: PropertyTypeInfo,
    pub index: usize,
    pub semantic: TextureType,
}

trait MaterialPropertyCaster {
    fn can_cast(&self) -> bool;
    fn cast(&self) -> Russult<PropertyTypeInfo>;
}

struct StringPropertyContent<'a> {
    property_info: &'a aiPropertyTypeInfo,
    key: &'a aiString,
    c_type: u32,
    index: u32,
    mat: &'a aiMaterial,
}

struct IntegerPropertyContent<'a> {
    property_info: &'a aiPropertyTypeInfo,
    key: &'a aiString,
    c_type: u32,
    index: u32,
    mat: &'a aiMaterial,
    data: &'a [u8],
}

struct FloatPropertyContent<'a> {
    property_info: &'a aiPropertyTypeInfo,
    key: &'a aiString,
    c_type: u32,
    index: u32,
    mat: &'a aiMaterial,
    data: &'a [u8],
}

struct BufferPropertyContent<'a> {
    property_info: &'a aiPropertyTypeInfo,
    data: &'a [u8],
}

impl<'a> MaterialPropertyCaster for BufferPropertyContent<'a> {
    fn can_cast(&self) -> bool {
        *self.property_info == aiPropertyTypeInfo_aiPTI_Buffer
    }

    fn cast(&self) -> Russult<PropertyTypeInfo> {
        Ok(PropertyTypeInfo::Buffer(self.data.to_vec()))
    }
}

impl<'a> MaterialPropertyCaster for IntegerPropertyContent<'a> {
    fn can_cast(&self) -> bool {
        *self.property_info == aiPropertyTypeInfo_aiPTI_Integer
    }

    fn cast(&self) -> Russult<PropertyTypeInfo> {
        let data_len = self.data.len();
        let mut max = data_len as u32 / 4;
        let result: Vec<i32> = vec![0; max as usize];

        if unsafe {
            aiGetMaterialIntegerArray(
                self.mat,
                self.key.data.as_ptr(),
                self.c_type,
                self.index,
                result.as_ptr() as *mut i32,
                &mut max,
            )
        } == aiReturn_aiReturn_SUCCESS
        {
            return Ok(PropertyTypeInfo::IntegerArray(result));
        }

        let key_string: String = self.key.into();
        Err(RussimpError::MeterialError(format!(
            "Error while parsing {} to f32",
            key_string
        )))
    }
}

impl<'a> MaterialPropertyCaster for FloatPropertyContent<'a> {
    fn can_cast(&self) -> bool {
        (*self.property_info & aiPropertyTypeInfo_aiPTI_Float) > 0
            || (*self.property_info & aiPropertyTypeInfo_aiPTI_Double) > 0
    }

    fn cast(&self) -> Russult<PropertyTypeInfo> {
        let data_len = self.data.len();
        let mut max = data_len as u32
            / if *self.property_info & aiPropertyTypeInfo_aiPTI_Double > 0 {
                8
            } else {
                4
            };
        let result: Vec<f32> = vec![0.0; max as usize];

        if unsafe {
            aiGetMaterialFloatArray(
                self.mat,
                self.key.data.as_ptr(),
                self.c_type,
                self.index,
                result.as_ptr() as *mut f32,
                &mut max,
            )
        } == aiReturn_aiReturn_SUCCESS
        {
            return Ok(PropertyTypeInfo::FloatArray(result));
        }

        let key_string: String = self.key.into();
        Err(RussimpError::MeterialError(format!(
            "Error while parsing {} to f32",
            key_string
        )))
    }
}

impl<'a> MaterialPropertyCaster for StringPropertyContent<'a> {
    fn can_cast(&self) -> bool {
        *self.property_info == aiPropertyTypeInfo_aiPTI_String
    }

    fn cast(&self) -> Russult<PropertyTypeInfo> {
        let mut content = MaybeUninit::zeroed();
        if unsafe {
            aiGetMaterialString(
                self.mat,
                self.key.data.as_ptr(),
                self.c_type,
                self.index,
                content.as_mut_ptr(),
            )
        } == aiReturn_aiReturn_SUCCESS
        {
            let ans = unsafe { content.assume_init() };
            return Ok(PropertyTypeInfo::String(ans.into()));
        }

        let key_string: String = self.key.into();
        Err(RussimpError::MeterialError(format!(
            "Error while parsing {} to string",
            key_string
        )))
    }
}

#[derive(Derivative, PartialEq)]
#[derivative(Debug)]
#[repr(u32)]
pub enum PropertyTypeInfo {
    // Force32Bit, aiPropertyTypeInfo__aiPTI_Force32Bit Not sure how to handle this
    Buffer(Vec<u8>),
    IntegerArray(Vec<i32>),
    FloatArray(Vec<f32>),
    String(String),
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

impl MaterialProperty {
    fn try_get_data_from_property(
        material: &aiMaterial,
        property: &aiMaterialProperty,
    ) -> Russult<PropertyTypeInfo> {
        let slice =
            slice_from_raw_parts(property.mData as *const u8, property.mDataLength as usize);
        let data = unsafe { slice.as_ref() }.unwrap();

        let casters: Vec<Box<dyn MaterialPropertyCaster>> = vec![
            Box::new(StringPropertyContent {
                key: &property.mKey,
                index: property.mIndex,
                c_type: property.mSemantic,
                mat: &material,
                property_info: &property.mType,
            }),
            Box::new(FloatPropertyContent {
                key: &property.mKey,
                index: property.mIndex,
                c_type: property.mSemantic,
                mat: &material,
                property_info: &property.mType,
                data,
            }),
            Box::new(IntegerPropertyContent {
                key: &property.mKey,
                index: property.mIndex,
                c_type: property.mSemantic,
                mat: &material,
                property_info: &property.mType,
                data,
            }),
            Box::new(BufferPropertyContent {
                data,
                property_info: &property.mType,
            }),
        ];

        for caster in casters {
            if caster.can_cast() {
                let data = caster.cast()?;
                return Ok(data);
            }
        }

        Err(RussimpError::MeterialError(
            "could not find caster for property type".to_string(),
        ))
    }

    pub fn new(material: &aiMaterial, property: &aiMaterialProperty) -> MaterialProperty {
        let data = Self::try_get_data_from_property(material, property).unwrap();

        MaterialProperty {
            key: property.mKey.into(),
            data,
            index: property.mIndex as usize,
            semantic: FromPrimitive::from_u32(property.mSemantic as u32).unwrap(),
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
    assert_eq!(
        "$mat.blend.mirror.glossAnisotropic",
        scene.materials[0].0[40].key.as_str()
    );
    assert_eq!(0, scene.materials[0].0[40].index);

    let ans_value = match &scene.materials[0].0[40].data {
        PropertyTypeInfo::Buffer(_) => 0.0,
        PropertyTypeInfo::IntegerArray(_) => 0.0,
        PropertyTypeInfo::FloatArray(x) => x[0],
        PropertyTypeInfo::String(_) => 0.0,
    };

    assert_eq!(1.0, ans_value);
    assert_eq!(TextureType::None, scene.materials[0].0[40].semantic);
}

#[test]
fn debug_material() {
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

    dbg!(&scene.materials);
}
