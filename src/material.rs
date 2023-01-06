use crate::{utils::get_base_type_vec_from_raw, sys::*, utils, RussimpError, Russult};
use derivative::Derivative;
use num_traits::FromPrimitive;
use std::{collections::HashMap, cell::RefCell, mem::MaybeUninit, ptr::slice_from_raw_parts, ffi::CStr, path::Path, rc::Rc};
use num_enum::TryFromPrimitive;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const FILENAME_PROPERTY: &str = "$tex.file";
const EMBEDDED_TEXNAME_PREFIX: &str = "*";

pub(crate) type Filename = String;

#[derive(Derivative, FromPrimitive, PartialEq, TryFromPrimitive, Clone, Eq, Hash, EnumIter, Copy)]
#[derivative(Debug)]
#[repr(u32)]
pub enum TextureType {
    #[num_enum(default)]
    None = aiTextureType_aiTextureType_NONE as _,
    Diffuse = aiTextureType_aiTextureType_DIFFUSE as _,
    Specular = aiTextureType_aiTextureType_SPECULAR as _,
    Ambient = aiTextureType_aiTextureType_AMBIENT as _,
    Emissive = aiTextureType_aiTextureType_EMISSIVE as _,
    Height = aiTextureType_aiTextureType_HEIGHT as _,
    Normals = aiTextureType_aiTextureType_NORMALS as _,
    Shininess = aiTextureType_aiTextureType_SHININESS as _,
    Opacity = aiTextureType_aiTextureType_OPACITY as _,
    Displacement = aiTextureType_aiTextureType_DISPLACEMENT as _,
    LightMap = aiTextureType_aiTextureType_LIGHTMAP as _,
    Reflection = aiTextureType_aiTextureType_REFLECTION as _,
    BaseColor = aiTextureType_aiTextureType_BASE_COLOR as _,
    NormalCamera = aiTextureType_aiTextureType_NORMAL_CAMERA as _,
    EmissionColor = aiTextureType_aiTextureType_EMISSION_COLOR as _,
    Metalness = aiTextureType_aiTextureType_METALNESS as _,
    Roughness = aiTextureType_aiTextureType_DIFFUSE_ROUGHNESS as _,
    AmbientOcclusion = aiTextureType_aiTextureType_AMBIENT_OCCLUSION as _,
    Unknown = aiTextureType_aiTextureType_UNKNOWN as _,
    Force32bit = aiTextureType__aiTextureType_Force32Bit as _,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Texture {
    pub height: u32,
    pub width: u32,
    pub filename: String,
    pub ach_format_hint: String,
    #[derivative(Debug = "ignore")]
    pub data: DataContent,
}

#[repr(C, packed)]
#[derive(Derivative, Copy, Clone)]
#[derivative(Debug)]
pub struct Texel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

impl From<&aiTexel> for Texel {
    fn from(texel: &aiTexel) -> Self {
        Texel {
            b: texel.b,
            g: texel.g,
            r: texel.r,
            a: texel.a,
        }
    }
}

#[derive(Clone)]
pub enum DataContent {
    Texel(Vec<Texel>),
    Bytes(Vec<u8>),
}

pub(crate) fn generate_materials(scene: &aiScene) -> Russult<Vec<Material>> {
    let textures = get_base_type_vec_from_raw(scene.mTextures, scene.mNumTextures);
    let materials = get_base_type_vec_from_raw(scene.mMaterials, scene.mNumMaterials);
    let properties = create_material_properties(&materials);
    let mut result = Vec::new();

    let mut converted_textures : HashMap<usize, Rc<RefCell<Texture>>> = HashMap::new();

    for (mat_index, &mat) in materials.iter().enumerate() {
        let mut material_textures : HashMap<TextureType, Rc<RefCell<Texture>>> = HashMap::new();

        for tex_type in TextureType::iter() {
            let material_filenames = get_textures_of_type_from_material(mat, tex_type)?;

            for material_filename in material_filenames {
                let embedded_textures = get_embedded_texture(&material_filename, &textures);

                if let Some(embedded_texture) = embedded_textures {
                    if let Some(tex) = converted_textures.get(&embedded_texture) {
                        material_textures.insert(tex_type, tex.clone());
                    } else {
                        let new_texture = create_texture_from(&textures[embedded_texture], true);
                        converted_textures.insert(embedded_texture, Rc::new(RefCell::new(new_texture)));
                        material_textures.insert(tex_type, converted_textures.get(&embedded_texture).unwrap().clone());
                    }
                }
            }
        }

        result.push(Material::new(properties[mat_index].clone(), material_textures));
    }

    Ok(result)
}

fn get_textures_of_type_from_material(
    material: &aiMaterial,
    texture_type: TextureType,
) -> Russult<Vec<Filename>> {
    let texture_type_raw: aiTextureType = texture_type as _;

    let mut vec = Vec::new();

    for index in 0..unsafe { aiGetMaterialTextureCount(material, texture_type_raw) } {
        vec.push(get_texture_filename(material, texture_type_raw, index)?);
    }

    Ok(vec)
}

fn get_texture_filename(
    material: &aiMaterial,
    texture_type: aiTextureType,
    index: u32,
) -> Russult<String> {
    let mut path = MaybeUninit::uninit();
    let mut texture_mapping = MaybeUninit::uninit();
    let mut uv_index = MaybeUninit::uninit();
    let mut blend = MaybeUninit::uninit();
    let mut op = MaybeUninit::uninit();
    let mut map_mode: [u32; 2] = [0, 0];

    let mut flags = MaybeUninit::uninit();

    if unsafe {
        aiGetMaterialTexture(
            material,
            texture_type,
            index,
            path.as_mut_ptr(),
            texture_mapping.as_mut_ptr(),
            uv_index.as_mut_ptr(),
            blend.as_mut_ptr(),
            op.as_mut_ptr(),
            map_mode.as_mut_ptr() as *mut _,
            flags.as_mut_ptr(),
        )
    } == aiReturn_aiReturn_SUCCESS
    {
        let filename : String = unsafe { path.assume_init() }.into();

        return Ok(filename);
    }

    Err(RussimpError::TextureNotFound)
}

fn create_texture_from(texture: &aiTexture, is_embedded: bool) -> Texture {
    let ach_format_hint = unsafe { CStr::from_ptr(texture.achFormatHint.as_ptr()) }.to_str().unwrap().to_string();

    let data = if is_embedded {
        let compressed_bytes =
            slice_from_raw_parts(texture.pcData as *const u8, texture.mWidth as usize);
        DataContent::Bytes(unsafe { compressed_bytes.as_ref() }.unwrap().to_vec())
    } else {
        DataContent::Texel(utils::get_vec(texture.pcData, texture.mWidth * texture.mHeight))
    };

    Texture {
        height: texture.mHeight,
        width: texture.mWidth,
        filename: texture.mFilename.into(),
        ach_format_hint,
        data,
    }
}


fn get_embedded_texture(file_name: &String, textures: &Vec<&aiTexture>) -> Option<usize> {
    if file_name.starts_with(EMBEDDED_TEXNAME_PREFIX) {
        let temp = file_name.split_at(1).1.to_string();
        let index = temp.parse::<usize>().unwrap();
        if textures.len() <= index {
            return None;
        }

        return Some(index);
    }

    let path = Path::new(file_name.as_str());
    if path.file_name().is_none() {
        return None;
    }

    for (tex_index, &texture) in textures.iter().enumerate() {
        let texture_filename: String = texture.mFilename.into();
        let texture_filepath = Path::new(texture_filename.as_str());

        if let Some(texture_name) = texture_filepath.file_name() {
            if let Some(name) = path.file_name() {
                if texture_name.eq(name) {
                    return Some(tex_index);
                }
            }
        }
    }

    None
}

fn create_material_properties(materials: &Vec<&aiMaterial>) -> Vec<Vec<MaterialProperty>> {
    let mut material_properties = Vec::new();

    for &i in materials {
        let properties = get_properties(i);

        material_properties.push(properties);
    }

    material_properties
}

fn get_properties(material: &aiMaterial) -> Vec<MaterialProperty> {
    let properties = get_base_type_vec_from_raw(material.mProperties, material.mNumProperties);
    let mut result = Vec::new();

    for item in properties {
        let material_property = MaterialProperty::new(material, item);
        result.push( material_property);
    }

    result
}

#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct Material {
    pub properties: Vec<MaterialProperty>,
    pub textures: HashMap<TextureType, Rc<RefCell<Texture>>>,
}

impl Material {
    fn new(properties: Vec<MaterialProperty>, textures: HashMap<TextureType, Rc<RefCell<Texture>>>) -> Self {
        Self {
            properties,
            textures,
        }
    }
}

#[derive(Derivative, Clone)]
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

#[derive(Derivative, PartialEq, Clone)]
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
            PostProcess::ValidateDataStructure,
        ],
    )
        .unwrap();

    dbg!(&scene.materials);
}

#[test]
fn filenames_available_for_textures() {
    use crate::{
        scene::{PostProcess, Scene},
    };

    let current_directory_buf =
        utils::get_model("models/GLTF2/BoxTextured-GLTF/BoxTextured.gltf");

    let scene = Scene::from_file(
        current_directory_buf.as_str(),
        vec![PostProcess::ValidateDataStructure],
    )
        .unwrap();

    assert_eq!(0, scene.materials[0].textures.len());
    assert_eq!(0, scene.materials[1].textures.len());

    let properties_first_material : Vec<&MaterialProperty> = scene.materials[0].properties.iter().filter(|x| x.key.eq(&FILENAME_PROPERTY.to_string())).collect();
    let properties_second_material : Vec<&MaterialProperty> = scene.materials[1].properties.iter().filter(|x| x.key.eq(&FILENAME_PROPERTY.to_string())).collect();

    assert!(properties_first_material.iter().any(|&x| x.semantic == TextureType::Diffuse));
    assert!(properties_first_material.iter().any(|&x| x.semantic == TextureType::BaseColor));
    assert_eq!(0, properties_second_material.len())
}

#[test]
fn read_embedded_texture_works_as_expected() {
    use crate::{
        scene::{PostProcess, Scene},
        material::TextureType::*,
    };

    let current_directory_buf =
        utils::get_model("models/GLTF2/BoxTextured-GLTF-Embedded/BoxTextured.gltf");

    let scene = Scene::from_file(
        current_directory_buf.as_str(),
        vec![PostProcess::ValidateDataStructure],
    )
    .unwrap();

    let texture = scene.materials[0].textures.get(&Diffuse).unwrap();

    let temp = texture.borrow();

    assert!(matches!(
        &temp.data,
        DataContent::Bytes(x) if x.len() > 0
    ));
}
