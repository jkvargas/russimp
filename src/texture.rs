use crate::{sys::*, *};
use derivative::Derivative;
use num_enum::TryFromPrimitive;
use num_traits::ToPrimitive;
use std::{
    collections::HashMap, ffi::CStr, mem::MaybeUninit, ops::BitAnd, ptr::slice_from_raw_parts,
};

const EMBEDDED_TEXNAME_PREFIX: &str = "*";

#[derive(Derivative, FromPrimitive, PartialEq, TryFromPrimitive, Clone, Eq, Hash)]
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

#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct Texture {
    pub path: String,
    pub texture_mapping: u32,
    pub uv_index: u32,
    pub blend: f32,
    pub op: u32,
    pub map_mode: Vec<u32>,
    pub flags: u32,
    pub height: u32,
    pub width: u32,
    pub ach_format_hint: String,
    #[derivative(Debug = "ignore")]
    pub data: Option<DataContent>,
}

pub enum DataContent {
    Texel(Vec<Texel>),
    Bytes(Vec<u8>),
}

struct TextureComponent {
    path: String,
    texture_mapping: u32,
    uv_index: u32,
    blend: f32,
    op: u32,
    map_mode: Vec<u32>,
    flags: u32,
}

impl TextureComponent {
    fn get_texture(
        material: &aiMaterial,
        texture_type: u32,
        index: u32,
    ) -> Russult<TextureComponent> {
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
                texture_type as _,
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
            let filename = unsafe { path.assume_init() }.into();

            let comp = TextureComponent::new(
                filename,
                unsafe { texture_mapping.assume_init() as _ },
                unsafe { uv_index.assume_init() },
                unsafe { blend.assume_init() },
                unsafe { op.assume_init() as _ },
                map_mode.to_vec(),
                unsafe { flags.assume_init() },
            );

            return Ok(comp);
        }

        Err(RussimpError::TextureNotFound)
    }

    fn get_textures_of_type_from_material(
        material: &aiMaterial,
        texture_type: TextureType,
    ) -> Vec<TextureComponent> {
        let texture_type_raw: aiTextureType = texture_type as _;

        let mut vec = Vec::new();

        for index in 0..unsafe { aiGetMaterialTextureCount(material, texture_type_raw) } {
            if let Ok(res) = Self::get_texture(material, texture_type_raw as _, index) {
                vec.push(res);
            }
        }

        vec
    }

    pub(crate) fn get_all_textures(
        material: &aiMaterial,
    ) -> HashMap<TextureType, Vec<TextureComponent>> {
        let mut map = HashMap::new();

        Self::feed_texture_map(material, TextureType::Diffuse, &mut map);
        Self::feed_texture_map(material, TextureType::Specular, &mut map);
        Self::feed_texture_map(material, TextureType::Ambient, &mut map);
        Self::feed_texture_map(material, TextureType::Emissive, &mut map);
        Self::feed_texture_map(material, TextureType::Height, &mut map);
        Self::feed_texture_map(material, TextureType::Normals, &mut map);
        Self::feed_texture_map(material, TextureType::Shininess, &mut map);
        Self::feed_texture_map(material, TextureType::Opacity, &mut map);
        Self::feed_texture_map(material, TextureType::Displacement, &mut map);
        Self::feed_texture_map(material, TextureType::LightMap, &mut map);
        Self::feed_texture_map(material, TextureType::Reflection, &mut map);
        Self::feed_texture_map(material, TextureType::BaseColor, &mut map);
        Self::feed_texture_map(material, TextureType::NormalCamera, &mut map);
        Self::feed_texture_map(material, TextureType::EmissionColor, &mut map);
        Self::feed_texture_map(material, TextureType::Metalness, &mut map);
        Self::feed_texture_map(material, TextureType::Roughness, &mut map);
        Self::feed_texture_map(material, TextureType::AmbientOcclusion, &mut map);
        Self::feed_texture_map(material, TextureType::Unknown, &mut map);

        map
    }

    #[inline]
    fn feed_texture_map(
        material: &aiMaterial,
        texture_type: TextureType,
        map: &mut HashMap<TextureType, Vec<TextureComponent>>,
    ) {
        let val = Self::get_textures_of_type_from_material(material, texture_type.clone());

        if val.len() > 0 {
            map.insert(texture_type, val);
        }
    }

    pub fn new(
        path: String,
        texture_mapping: u32,
        uv_index: u32,
        blend: f32,
        op: u32,
        map_mode: Vec<u32>,
        flags: u32,
    ) -> TextureComponent {
        Self {
            path,
            texture_mapping,
            uv_index,
            blend,
            op,
            map_mode,
            flags,
        }
    }
}

#[derive(Derivative, FromPrimitive, PartialEq, TryFromPrimitive, Clone, Eq, Hash, ToPrimitive)]
#[derivative(Debug)]
#[repr(u32)]
pub enum TextureMapMode {
    Clamp = aiTextureMapMode_aiTextureMapMode_Clamp as _,
    Decal = aiTextureMapMode_aiTextureMapMode_Decal as _,
    Mirror = aiTextureMapMode_aiTextureMapMode_Mirror as _,
    Wrap = aiTextureMapMode_aiTextureMapMode_Wrap as _,
}

impl BitAnd<TextureMapMode> for TextureMapMode {
    type Output = u32;

    fn bitand(self, rhs: TextureMapMode) -> Self::Output {
        ToPrimitive::to_u32(&self).unwrap() & ToPrimitive::to_u32(&rhs).unwrap()
    }
}

impl BitAnd<TextureMapMode> for u32 {
    type Output = u32;

    fn bitand(self, rhs: TextureMapMode) -> Self::Output {
        self & ToPrimitive::to_u32(&rhs).unwrap()
    }
}

impl BitAnd<u32> for TextureMapMode {
    type Output = u32;

    fn bitand(self, rhs: u32) -> Self::Output {
        ToPrimitive::to_u32(&self).unwrap() & rhs
    }
}

impl Texture {
    pub(crate) fn get_textures_from_material(
        material: &aiMaterial,
        textures: &Vec<&aiTexture>,
    ) -> HashMap<TextureType, Vec<Texture>> {
        let texture_components = TextureComponent::get_all_textures(material);
        let mut map: HashMap<TextureType, Vec<Texture>> = HashMap::new();

        for (t_type, components) in texture_components {
            map.insert(
                t_type.clone(),
                components
                    .iter()
                    .map(|x| Texture::new(x, textures))
                    .collect(),
            );
        }

        map
    }

    fn new(texture_component: &TextureComponent, textures: &Vec<&aiTexture>) -> Self {
        let mut result = Self {
            path: texture_component.path.clone(),
            flags: texture_component.flags,
            // the clone is just for two elements
            map_mode: texture_component.map_mode.clone(),
            uv_index: texture_component.uv_index,
            texture_mapping: texture_component.texture_mapping,
            blend: texture_component.blend,
            op: texture_component.op,
            width: 0,
            height: 0,
            data: None,
            ach_format_hint: String::new(),
        };

        if Self::is_file_embedded(&texture_component.path) {
            let slice = &texture_component.path[EMBEDDED_TEXNAME_PREFIX.len()..];
            let texture_index: usize = std::str::FromStr::from_str(slice).unwrap();
            let texture = textures[texture_index];
            let content = unsafe { CStr::from_ptr(texture.achFormatHint.as_ptr()) };
            let ach_format_hint = content.to_str().unwrap().to_string();
            let data = Self::get_texels_and_buffer_from_embedded_file(&texture);

            result.data = Some(data);
            result.width = texture.mWidth;
            result.height = texture.mHeight;
            result.ach_format_hint = ach_format_hint;
        }

        result
    }

    fn get_texels_and_buffer_from_embedded_file(texture: &aiTexture) -> DataContent {
        if Self::is_embedded_file_compressed(texture) {
            DataContent::Bytes(Self::load_embedded_file(texture))
        } else {
            DataContent::Texel(Self::load_texels(texture))
        }
    }

    #[inline]
    fn is_file_embedded(file_path: &String) -> bool {
        file_path.starts_with(EMBEDDED_TEXNAME_PREFIX)
    }

    #[inline]
    fn is_embedded_file_compressed(texture: &aiTexture) -> bool {
        texture.mHeight == 0
    }

    #[inline]
    fn load_texels(texture: &aiTexture) -> Vec<Texel> {
        utils::get_vec(texture.pcData, texture.mWidth * texture.mHeight)
    }

    fn load_embedded_file(texture: &aiTexture) -> Vec<u8> {
        let compressed_bytes =
            slice_from_raw_parts(texture.pcData as *const u8, texture.mWidth as usize);
        unsafe { compressed_bytes.as_ref() }.unwrap().to_vec()
    }
}

#[test]
fn debug_texture() {
    use crate::scene::{PostProcess, Scene};

    let current_directory_buf =
        utils::get_model("models/GLTF2/BoxTextured-GLTF-Embedded/BoxTextured.gltf");

    let scene = Scene::from_file(
        current_directory_buf.as_str(),
        vec![PostProcess::ValidateDataStructure],
    )
    .unwrap();

    dbg!(&scene.materials);
}

#[test]
fn amount_of_textures() {
    use crate::{
        scene::{PostProcess, Scene},
        texture::TextureType::Diffuse,
    };

    let current_directory_buf =
        utils::get_model("models/GLTF2/BoxTextured-GLTF-Embedded/BoxTextured.gltf");

    let scene = Scene::from_file(
        current_directory_buf.as_str(),
        vec![PostProcess::ValidateDataStructure],
    )
    .unwrap();

    let textures = scene.materials[0].textures.get(&Diffuse).unwrap();
    assert_eq!(1, textures.len());

    assert!(matches!(
        textures[0].data.as_ref().unwrap(),
        DataContent::Bytes(_)
    ));
}

#[test]
fn map_modes_are_correct() {
    use crate::{
        scene::{PostProcess, Scene},
        texture::{
            TextureMapMode::{Clamp, Mirror},
            TextureType::Diffuse,
        },
    };

    let current_directory_buf = utils::get_model("models/GLTF2/BoxTextured-GLTF/BoxTextured.gltf");

    let scene = Scene::from_file(
        current_directory_buf.as_str(),
        vec![PostProcess::ValidateDataStructure],
    )
    .unwrap();

    let texture = &scene.materials[0].textures.get(&Diffuse).unwrap()[0];

    assert_ne!(texture.map_mode[0] & Mirror, 0);
    assert_ne!(texture.map_mode[1] & Clamp, 0);
}
