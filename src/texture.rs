use crate::{sys::*, *};
use derivative::Derivative;
use num_enum::TryFromPrimitive;
use std::{
    collections::HashMap, ffi::CStr, mem::MaybeUninit, ptr::slice_from_raw_parts, str::from_utf8,
};
use sys::aiMaterial;

#[derive(Derivative, FromPrimitive, PartialEq, TryFromPrimitive, Clone, Eq, Hash)]
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
    pub map_mode: u32,
    pub flags: u32,
    pub height: u32,
    pub width: u32,
    pub ach_format_hint: String,
    pub texel: Vec<Texel>,
    pub data: Vec<u8>,
}

struct TextureComponent {
    path: String,
    texture_mapping: u32,
    uv_index: u32,
    blend: f32,
    op: u32,
    map_mode: u32,
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
        let mut map_mode = MaybeUninit::uninit();
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
                map_mode.as_mut_ptr(),
                flags.as_mut_ptr(),
            )
        } == aiReturn_aiReturn_SUCCESS
        {
            let filename = unsafe { path.assume_init() }.into();

            let comp = TextureComponent::new(
                filename,
                unsafe { texture_mapping.assume_init() },
                unsafe { uv_index.assume_init() },
                unsafe { blend.assume_init() },
                unsafe { op.assume_init() },
                unsafe { map_mode.assume_init() },
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
        let texture_type_raw : aiTextureType = texture_type as u32;

        let mut vec = Vec::new();

        for mut index in 0..unsafe { aiGetMaterialTextureCount(material, texture_type_raw) } {
            if let Ok(res) = Self::get_texture(material, texture_type_raw, index) {
                index += 1;
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
        map_mode: u32,
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

impl Texture {
    pub(crate) fn get_textures_from_material(
        material: &aiMaterial,
        textures: &[aiTexture],
    ) -> HashMap<TextureType, Vec<Texture>> {
        let texture_components = TextureComponent::get_all_textures(material);
        let mut map: HashMap<TextureType, Vec<Texture>> = HashMap::new();

        for (t_type, components) in texture_components {
            map.insert(
                t_type.clone(),
                components
                    .iter()
                    .map(|x| Texture::new(x, textures))
                    .collect()
            );
        }

        map
    }

    #[inline]
    fn get_embedded_prefix<'a>() -> &'a str {
        from_utf8(AI_EMBEDDED_TEXNAME_PREFIX.as_ref()).unwrap()
    }

    fn new(texture_component: &TextureComponent, textures: &[aiTexture]) -> Self {
        let slice = &texture_component.path[Self::get_embedded_prefix().len()..];
        let texture_index : usize = std::str::FromStr::from_str(slice).unwrap();
        let texture = textures[texture_index];
        let content = unsafe { CStr::from_ptr(texture.achFormatHint.as_ptr()) };
        let ach_format_hint = content.to_str().unwrap().to_string();
        let (texel, data) = Self::get_texels_and_buffer(texture_component, &texture);

        Self {
            path: texture_component.path.clone(),
            flags: texture_component.flags,
            map_mode: texture_component.map_mode,
            uv_index: texture_component.uv_index,
            texture_mapping: texture_component.texture_mapping,
            blend: texture_component.blend,
            op: texture_component.op,
            width: texture.mWidth,
            height: texture.mHeight,
            data,
            texel,
            ach_format_hint,
        }
    }

    fn get_texels_and_buffer(
        texture_component: &TextureComponent,
        texture: &aiTexture,
    ) -> (Vec<Texel>, Vec<u8>) {
        if Self::is_file_embedded(&texture_component.path) {
            if Self::is_embedded_file_compressed(texture) {
                (
                    vec![],
                    Self::load_embedded_file(texture),
                )
            } else {
                (Self::load_texels(texture), vec![])
            }
        } else {
            (vec![], vec![])
        }
    }

    #[inline]
    fn is_file_embedded(file_path: &String) -> bool {
        file_path.starts_with(Self::get_embedded_prefix())
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

    let current_directory_buf = utils::get_model("models/GLTF2/BoxTextured.gltf");

    let scene = Scene::from_file(current_directory_buf.as_str(),
                                 vec![
                                     PostProcess::Triangulate,
                                     PostProcess::FlipUVs,
                                     PostProcess::EmbedTextures
                                 ]
    ).unwrap();

    dbg!(&scene.materials);
}
