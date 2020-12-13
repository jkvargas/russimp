use crate::{
    FromRaw,
    sys,
    scene::{
        Scene,
        PostProcessSteps,
    },
};

pub struct MeshMorphKey<'a> {
    mesh_morph_key: &'a aiMeshMorphKey,
    pub time: f64,
    pub values: Vec<&'a u32>,
    pub weights: Vec<&'a f64>,
}

impl<'a> FromRaw for MeshMorphKey<'a> {}

impl<'a> Into<MeshMorphKey<'a>> for &'a aiMeshMorphKey {
    fn into(self) -> MeshMorphKey<'a> {
        MeshMorphKey {
            mesh_morph_key: self,
            time: self.mTime,
            values: MeshMorphKey::get_rawvec(self.mValues, self.mNumValuesAndWeights),
            weights: MeshMorphKey::get_rawvec(self.mWeights, self.mNumValuesAndWeights),
        }
    }
}

pub struct MeshMorphAnim<'a> {
    mesh_morph_anim: &'a aiMeshMorphAnim,
    pub keys: Vec<MeshMorphKey<'a>>,
    pub name: String,
}

impl<'a> FromRaw for MeshMorphAnim<'a> {}

impl<'a> Into<MeshMorphAnim<'a>> for &'a aiMeshMorphAnim {
    fn into(self) -> MeshMorphAnim<'a> {
        MeshMorphAnim {
            mesh_morph_anim: self,
            keys: MeshMorphAnim::get_vec(self.mKeys, self.mNumKeys),
            name: self.mName.into(),
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

impl<'a> FromRaw for NodeAnim<'a> {}

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

pub struct MeshAnim<'a> {
    mesh_anim: &'a aiMeshAnim,
    name: String,
    keys: Vec<&'a aiMeshKey>,
}

impl<'a> FromRaw for MeshAnim<'a> {}

impl<'a> Into<MeshAnim<'a>> for &'a aiMeshAnim {
    fn into(self) -> MeshAnim<'a> {
        MeshAnim {
            mesh_anim: self,
            name: self.mName.into(),
            keys: MeshAnim::get_vec(self.mKeys, self.mNumKeys),
        }
    }
}

pub struct Animation<'a> {
    animation: &'a aiAnimation,
    pub name: String,
    pub channels: Vec<NodeAnim<'a>>,
    pub duration: f64,
    pub morph_mesh_channels: Vec<MeshMorphAnim<'a>>,
    pub mesh_channels: Vec<MeshAnim<'a>>,
    pub ticks_per_second: f64,
}

impl<'a> FromRaw for Animation<'a> {}

impl<'a> Into<Animation<'a>> for &'a aiAnimation {
    fn into(self) -> Animation<'a> {
        Animation {
            animation: self,
            name: self.mName.into(),
            channels: Animation::get_vec_from_raw(self.mChannels, self.mNumChannels),
            duration: self.mDuration,
            morph_mesh_channels: Animation::get_vec_from_raw(self.mMorphMeshChannels, self.mNumMorphMeshChannels),
            mesh_channels: Animation::get_vec_from_raw(self.mMeshChannels, self.mNumMeshChannels),
            ticks_per_second: self.mTicksPerSecond,
        }
    }
}

#[test]
fn camera_roll_animation_read() {
    let current_directory_buf = std::env::current_dir().unwrap().join("models/3DS/CameraRollAnim.3ds");

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