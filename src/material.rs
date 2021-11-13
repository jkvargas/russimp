use crate::utils::get_base_type_vec_from_raw;
use crate::{sys::*, texture::Texture, texture::TextureType, utils, RussimpError, Russult};
use derivative::Derivative;
use num_traits::FromPrimitive;
use std::{collections::HashMap, mem::MaybeUninit, ptr::slice_from_raw_parts};

pub(crate) struct MaterialFactory<'a> {
    materials: Vec<&'a aiMaterial>,
    textures: Vec<&'a aiTexture>,
}

impl<'a> MaterialFactory<'a> {
    pub(crate) fn new(scene: &aiScene) -> Russult<Self> {
        let textures = utils::get_base_type_vec_from_raw(scene.mTextures, scene.mNumTextures);
        let materials = utils::get_base_type_vec_from_raw(scene.mMaterials, scene.mNumMaterials);

        Ok(Self {
            textures,
            materials,
        })
    }

    pub(crate) fn create_materials(&self) -> Vec<Material> {
        let mut vec = Vec::new();

        for mat in &self.materials {
            let textures = Texture::get_textures_from_material(*mat, &self.textures);
            let material = Material::new(*mat, textures);
            vec.push(material);
        }

        vec
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Material {
    pub properties: Vec<MaterialProperty>,
    pub textures: HashMap<TextureType, Vec<Texture>>,
}

impl Material {
    fn new(material: &aiMaterial, textures: HashMap<TextureType, Vec<Texture>>) -> Self {
        Self {
            properties: Self::get_properties(material),
            textures,
        }
    }

    fn get_properties(material: &aiMaterial) -> Vec<MaterialProperty> {
        let properties = get_base_type_vec_from_raw(material.mProperties, material.mNumProperties);
        let mut result = Vec::new();

        for item in properties {
            result.push(MaterialProperty::new(material, item));
        }

        result
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct MaterialProperty {
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
        let mut content = MaybeUninit::uninit();
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
    use crate::{
        scene::{PostProcess, Scene},
        utils,
    };

    let box_file_path = utils::get_model("models/BLEND/box.blend");

    let scene = Scene::from_file(
        box_file_path.as_str(),
        vec![PostProcess::ValidateDataStructure],
    )
    .unwrap();

    assert_eq!(1, scene.materials.len());
    assert_eq!(41, scene.materials[0].properties.len());
    assert_eq!(
        "$mat.blend.mirror.glossAnisotropic",
        scene.materials[0].properties[40].key.as_str()
    );
    assert_eq!(0, scene.materials[0].properties[40].index);

    let ans_value = match &scene.materials[0].properties[40].data {
        PropertyTypeInfo::Buffer(_) => 0.0,
        PropertyTypeInfo::IntegerArray(_) => 0.0,
        PropertyTypeInfo::FloatArray(x) => x[0],
        PropertyTypeInfo::String(_) => 0.0,
    };

    assert_eq!(1.0, ans_value);
    assert_eq!(
        TextureType::None,
        scene.materials[0].properties[40].semantic
    );
}

#[test]
fn debug_material() {
    use crate::{
        scene::{PostProcess, Scene},
        utils,
    };

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
