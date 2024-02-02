use std::ffi::CStr;

use russimp_sys::{
    aiCreatePropertyStore, aiMatrix4x4, aiPropertyStore, aiReleasePropertyStore,
    aiSetImportPropertyFloat, aiSetImportPropertyInteger, aiSetImportPropertyMatrix,
    aiSetImportPropertyString, aiString,
};

pub enum Property {
    String(&'static str),
    Float(f32),
    Integer(i32),
    Matrix([[f32; 4]; 4]),
}

pub struct PropertyStore {
    ptr: *mut aiPropertyStore,
}

impl Drop for PropertyStore {
    #[inline]
    fn drop(&mut self) {
        unsafe { aiReleasePropertyStore(self.ptr) };
    }
}

impl Default for PropertyStore {
    fn default() -> Self {
        let ptr = unsafe { aiCreatePropertyStore() };
        Self { ptr }
    }
}

impl PropertyStore {
    pub fn set_integer(&mut self, name: &[u8], value: i32) {
        let c_name = CStr::from_bytes_until_nul(name).unwrap();
        unsafe { aiSetImportPropertyInteger(self.ptr, c_name.as_ptr(), value) };
    }

    pub fn set_float(&mut self, name: &[u8], value: f32) {
        let c_name = CStr::from_bytes_until_nul(name).unwrap();
        unsafe { aiSetImportPropertyFloat(self.ptr, c_name.as_ptr(), value) };
    }

    pub fn set_string(&mut self, name: &[u8], value: &str) {
        let c_name = CStr::from_bytes_until_nul(name).unwrap();
        let bytes: &[::std::os::raw::c_char] = unsafe { std::mem::transmute(value.as_bytes()) };
        let mut string = aiString {
            length: bytes.len() as u32,
            data: [0; 1024],
        };
        let n = std::cmp::min(string.data.len(), bytes.len());
        string.data[0..n].copy_from_slice(&bytes[0..n]);
        unsafe { aiSetImportPropertyString(self.ptr, c_name.as_ptr(), &string as *const aiString) };
    }

    pub fn set_matrix(&mut self, name: &[u8], value: [[f32; 4]; 4]) {
        let c_name = CStr::from_bytes_until_nul(name).unwrap();
        // NOTE: Assuming column-major matrix
        let matrix = aiMatrix4x4 {
            a1: value[0][0],
            a2: value[1][0],
            a3: value[2][0],
            a4: value[3][0],
            b1: value[0][1],
            b2: value[1][1],
            b3: value[2][1],
            b4: value[3][1],
            c1: value[0][2],
            c2: value[1][2],
            c3: value[2][2],
            c4: value[3][2],
            d1: value[0][3],
            d2: value[1][3],
            d3: value[2][3],
            d4: value[3][3],
        };
        unsafe {
            aiSetImportPropertyMatrix(self.ptr, c_name.as_ptr(), &matrix as *const aiMatrix4x4)
        };
    }

    pub(crate) fn as_ptr(&self) -> *mut aiPropertyStore {
        self.ptr
    }
}

impl<T: Iterator<Item = (&'static [u8], Property)>> From<T> for PropertyStore {
    fn from(value: T) -> Self {
        let mut props = Self::default();
        for (name, prop) in value {
            match prop {
                Property::String(v) => props.set_string(name, v),
                Property::Float(v) => props.set_float(name, v),
                Property::Integer(v) => props.set_integer(name, v),
                Property::Matrix(v) => props.set_matrix(name, v),
            }
        }
        props
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::node::Node;
    use crate::scene::{PostProcess, PostProcessSteps, Scene};
    use crate::sys::{
        AI_CONFIG_IMPORT_COLLADA_IGNORE_UP_DIRECTION, AI_CONFIG_IMPORT_COLLADA_USE_COLLADA_NAMES,
        AI_CONFIG_IMPORT_FBX_PRESERVE_PIVOTS, AI_CONFIG_IMPORT_FBX_READ_WEIGHTS,
        AI_CONFIG_PP_OG_EXCLUDE_LIST, AI_CONFIG_PP_PTV_ADD_ROOT_TRANSFORMATION,
        AI_CONFIG_PP_PTV_ROOT_TRANSFORMATION,
    };
    use crate::{utils, RussimpError, Russult};

    use super::{Property, PropertyStore};

    fn load_scene_with_props(
        model: &str,
        flags: Option<PostProcessSteps>,
        props: &PropertyStore,
        from_buffer: bool,
    ) -> Russult<Scene> {
        let model = utils::get_model(model);
        let flags = flags.unwrap_or(vec![]);
        if from_buffer {
            let model_path = Path::new(model.as_str());
            let buffer = std::fs::read(model.as_str())
                .map_err(|_| RussimpError::Import(String::from("Failed to read file")))?;
            let file_name = model_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            Scene::from_buffer_with_props(buffer.as_slice(), flags, file_name, props)
        } else {
            Scene::from_file_with_props(model.as_str(), flags, props)
        }
    }

    #[test]
    fn import_fbx_without_preserving_pivots() {
        fn traverse_check_fbx_node(node: &Node) -> bool {
            if node.name.ends_with("_$AssimpFbx$_RotationPivot")
                || node.name.ends_with("_$AssimpFbx$_RotationOffset")
                || node.name.ends_with("_$AssimpFbx$_PreRotation")
                || node.name.ends_with("_$AssimpFbx$_PostRotation")
                || node.name.ends_with("_$AssimpFbx$_ScalingPivot")
                || node.name.ends_with("_$AssimpFbx$_ScalingOffset")
                || node.name.ends_with("_$AssimpFbx$_Translation")
                || node.name.ends_with("_$AssimpFbx$_Scaling")
                || node.name.ends_with("_$AssimpFbx$_Rotation")
            {
                return false;
            }
            for child in node.children.borrow().iter() {
                if !traverse_check_fbx_node(&*child) {
                    return false;
                }
            }
            true
        }

        let props: PropertyStore = [(
            AI_CONFIG_IMPORT_FBX_PRESERVE_PIVOTS as &[u8],
            Property::Integer(0),
        )]
        .into_iter()
        .into();
        let scene = load_scene_with_props("models/FBX/y_bot_run.fbx", None, &props, false).unwrap();

        // NOTE: A scene with collapsed FBX transforms should not contain
        // nodes with names like "<OriginalName>_$AssimpFbx$_<TransformName>"
        // https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXImportSettings.h#L141
        if let Some(root) = scene.root {
            assert!(traverse_check_fbx_node(&root));
        }
    }

    #[test]
    fn import_fbx_without_weights() {
        let props: PropertyStore = [(
            AI_CONFIG_IMPORT_FBX_READ_WEIGHTS as &[u8],
            Property::Integer(0),
        )]
        .into_iter()
        .into();
        let scene = load_scene_with_props("models/FBX/y_bot_run.fbx", None, &props, true).unwrap();
        assert_eq!(scene.meshes.len(), 2);
        for mesh in &scene.meshes {
            for bone in &mesh.bones {
                assert_eq!(bone.weights.len(), 0);
            }
        }
    }

    #[test]
    fn import_collada_with_ignore_up_direction() {
        let props: PropertyStore = [(
            AI_CONFIG_IMPORT_COLLADA_IGNORE_UP_DIRECTION as &[u8],
            Property::Integer(1),
        )]
        .into_iter()
        .into();
        let scene =
            load_scene_with_props("models/COLLADA/blender_cube.dae", None, &props, false).unwrap();

        // NOTE: Ignoring the COLLADA file's UP direction should yield
        // an identity matrix as the root node transformation, meaning
        // we are now using blender coordinate system (+Z up instead of +Y up)
        if let Some(root) = scene.root {
            let is_identity = root.transformation.a1 == 1.0
                && root.transformation.a2 == 0.0
                && root.transformation.a3 == 0.0
                && root.transformation.a4 == 0.0
                && root.transformation.b1 == 0.0
                && root.transformation.b2 == 1.0
                && root.transformation.b3 == 0.0
                && root.transformation.b4 == 0.0
                && root.transformation.c1 == 0.0
                && root.transformation.c2 == 0.0
                && root.transformation.c3 == 1.0
                && root.transformation.c4 == 0.0
                && root.transformation.d1 == 0.0
                && root.transformation.d2 == 0.0
                && root.transformation.d3 == 0.0
                && root.transformation.d4 == 1.0;
            assert!(is_identity);
        }
    }

    #[test]
    fn import_collada_with_use_collada_names() {
        let props: PropertyStore = [(
            AI_CONFIG_IMPORT_COLLADA_USE_COLLADA_NAMES as &[u8],
            Property::Integer(1),
        )]
        .into_iter()
        .into();
        let scene =
            load_scene_with_props("models/COLLADA/blender_cube.dae", None, &props, true).unwrap();

        // NOTE: Importing a COLLADA file with this option enabled
        // should yield the real mesh names like: "Cube.001"
        // instead of "Cube_001-mesh" as the importer should use
        // the geometry's `name` property instead of `id`
        assert_eq!(scene.meshes.len(), 1);
        assert_eq!(scene.meshes[0].name, "Cube.001");
    }

    #[test]
    fn import_pp_ptv_root_transformation() {
        let props: PropertyStore = [
            (
                AI_CONFIG_IMPORT_COLLADA_IGNORE_UP_DIRECTION as &[u8],
                Property::Integer(1),
            ),
            (
                AI_CONFIG_PP_PTV_ADD_ROOT_TRANSFORMATION as &[u8],
                Property::Integer(1),
            ),
            (
                AI_CONFIG_PP_PTV_ROOT_TRANSFORMATION as &[u8],
                Property::Matrix([
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 0.0, -1.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ]),
            ),
        ]
        .into_iter()
        .into();
        let scene = load_scene_with_props(
            "models/COLLADA/blender_plane.dae",
            Some(vec![PostProcess::PreTransformVertices]),
            &props,
            false,
        )
        .unwrap();

        // NOTE: The exported blender plane's normal is facing +Z (ignoring COLLADA up direction)
        // If we pre-transform its vertices with the above matrix,
        // its normal should be aligned with the Y axis,
        // i.e. all the Y coordinates of its vertices should be equal to 0
        assert_eq!(scene.meshes.len(), 1);
        for vertex in &scene.meshes[0].vertices {
            assert_eq!(vertex.y, 0.0);
        }
    }

    #[test]
    fn import_pp_og_exclude_list() {
        fn traverse_find_bone_end(node: &Node) -> bool {
            if node.name.as_str() == "Bone_end" {
                return true;
            }
            for child in node.children.borrow().iter() {
                if traverse_find_bone_end(&*child) {
                    return true;
                }
            }
            return false;
        }

        let props: PropertyStore = [(
            AI_CONFIG_PP_OG_EXCLUDE_LIST as &[u8],
            Property::String("Bone_end"),
        )]
        .into_iter()
        .into();
        let scene = load_scene_with_props(
            "models/FBX/cube_armature.fbx",
            Some(vec![PostProcess::OptimizeGraph]),
            &props,
            true,
        )
        .unwrap();

        // NOTE: Exported FBX file contains a cube with a single bone.
        // The bone's end is also exported and is technically unused,
        // but setting this option should preserve it in the hierarchy.
        if let Some(root) = &scene.root {
            assert!(traverse_find_bone_end(&*root));
        }
    }
}
