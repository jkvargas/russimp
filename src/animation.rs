use russimp_sys::{aiAnimation, aiNodeAnim, aiVectorKey, aiVector3D, aiQuatKey, aiMeshMorphAnim};
use crate::{
    FromRaw,
    scene::{Scene, PostProcessSteps}
};

pub struct Animation<'a> {
    animation: &'a aiAnimation,
    pub name: String,
    pub channels: Vec<NodeAnim<'a>>,
    pub duration: f64,
    pub morph_mesh_channels: Vec<MeshMorphAnim<'a>>,
}

pub struct MeshMorphAnim<'a> {
    mesh_morph_anim: &'a aiMeshMorphAnim
}

impl<'a> Into<MeshMorphAnim<'a>> for &'a aiMeshMorphAnim {
    fn into(self) -> MeshMorphAnim<'a> {
        MeshMorphAnim {
            mesh_morph_anim: self
        }
    }
}

impl<'a> Into<Animation<'a>> for &'a aiAnimation {
    fn into(self) -> Animation<'a> {
        Animation {
            animation: self,
            name: self.mName.into(),
            channels: Animation::get_vec_from_raw(self.mChannels, self.mNumChannels),
            duration: self.mDuration,
            morph_mesh_channels: Animation::get_vec_from_raw(self.mMorphMeshChannels, self.mNumMorphMeshChannels),
        }
    }
}

pub struct NodeAnim<'a> {
    anim_node: &'a aiNodeAnim,
    pub name: String,
    pub position_keys: Vec<&'a aiVectorKey>,
    pub rotation_keys: Vec<&'a aiQuatKey>,
    pub scaling_keys: Vec<&'a aiVectorKey>,
    pub post_state: u32,
    pub pre_state: u32,
}

impl<'a> Into<NodeAnim<'a>> for &'a aiNodeAnim {
    fn into(self) -> NodeAnim<'a> {
        NodeAnim {
            anim_node: self,
            name: self.mNodeName.into(),
            position_keys: NodeAnim::get_vec(self.mPositionKeys, self.mNumPositionKeys),
            rotation_keys: NodeAnim::get_vec(self.mRotationKeys, self.mNumRotationKeys),
            scaling_keys: NodeAnim::get_vec(self.mScalingKeys, self.mNumScalingKeys),
            post_state: self.mPostState,
            pre_state: self.mPreState,
        }
    }
}

impl<'a> FromRaw for Animation<'a> {}

impl<'a> FromRaw for NodeAnim<'a> {}

#[test]
fn camera_roll_animation_read() {
    let current_directory_buf = std::env::current_dir().unwrap().join("russimp-sys/assimp/test/models/3DS/CameraRollAnim.3ds");

    let scene = Scene::from(current_directory_buf.to_str().unwrap(),
                            vec![PostProcessSteps::CalcTangentSpace,
                                 PostProcessSteps::Triangulate,
                                 PostProcessSteps::JoinIdenticalVertices,
                                 PostProcessSteps::SortByPType]).unwrap();

    assert_eq!(1, scene.animations.len());
    assert_eq!("3DSMasterAnim".to_string(), scene.animations[0].name);
    assert_eq!(1, scene.animations[0].channels.len());
    assert_eq!("Camera01".to_string(), scene.animations[0].channels[0].name);

    assert_eq!(0, scene.animations[0].channels[0].pre_state);
    assert_eq!(0, scene.animations[0].channels[0].post_state);
    assert_eq!(0.0, scene.animations[0].channels[0].rotation_keys[0].mTime);
    assert_eq!(0.9999999, scene.animations[0].channels[0].rotation_keys[0].mValue.w);
    assert_eq!(-0.00046456736, scene.animations[0].channels[0].rotation_keys[0].mValue.x);
    assert_eq!(0.0, scene.animations[0].channels[0].rotation_keys[0].mValue.y);
    assert_eq!(0.0, scene.animations[0].channels[0].rotation_keys[0].mValue.z);

    assert_eq!(120.0, scene.animations[0].channels[0].rotation_keys[1].mTime);
    assert_eq!(0.7806558, scene.animations[0].channels[0].rotation_keys[1].mValue.w);
    assert_eq!(-0.6249612, scene.animations[0].channels[0].rotation_keys[1].mValue.x);
    assert_eq!(0.0, scene.animations[0].channels[0].rotation_keys[1].mValue.y);
    assert_eq!(0.0, scene.animations[0].channels[0].rotation_keys[1].mValue.z);

    assert_eq!(0.0, scene.animations[0].channels[0].scaling_keys[0].mTime);
    assert_eq!(1.0, scene.animations[0].channels[0].scaling_keys[0].mValue.x);
    assert_eq!(1.0, scene.animations[0].channels[0].scaling_keys[0].mValue.y);
    assert_eq!(1.0, scene.animations[0].channels[0].scaling_keys[0].mValue.z);

    // position keys
    assert_eq!(1, scene.animations[0].channels[0].position_keys.len());
    assert_eq!(0.0, scene.animations[0].channels[0].position_keys[0].mTime);
    assert_eq!(-153.0771, scene.animations[0].channels[0].position_keys[0].mValue.x);
    assert_eq!(3.272005, scene.animations[0].channels[0].position_keys[0].mValue.y);
    assert_eq!(22.777624, scene.animations[0].channels[0].position_keys[0].mValue.z);

    assert_eq!(120.0, scene.animations[0].duration);
    assert_eq!(0, scene.animations[0].morph_mesh_channels.len());
}