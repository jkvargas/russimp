use crate::{
    FromRaw,
    sys::{
        aiMeshMorphKey,
        aiMeshMorphAnim,
        aiNodeAnim,
        aiQuatKey,
        aiVectorKey,
        aiMeshAnim,
        aiMeshKey,
        aiAnimation,
    }, scene::{
        Scene,
        PostProcessSteps,
    },
    get_model};

use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct MeshMorphKey {
    pub time: f64,
    pub values: Vec<u32>,
    pub weights: Vec<f64>,
}

impl FromRaw for MeshMorphKey {}

impl Into<MeshMorphKey> for &aiMeshMorphKey {
    fn into(self) -> MeshMorphKey {
        MeshMorphKey {
            time: self.mTime,
            values: MeshMorphKey::get_rawvec(self.mValues, self.mNumValuesAndWeights),
            weights: MeshMorphKey::get_rawvec(self.mWeights, self.mNumValuesAndWeights),
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct MeshMorphAnim {
    pub keys: Vec<MeshMorphKey>,
    pub name: String,
}

impl FromRaw for MeshMorphAnim {}

impl Into<MeshMorphAnim> for &aiMeshMorphAnim {
    fn into(self) -> MeshMorphAnim {
        MeshMorphAnim {
            keys: MeshMorphAnim::get_vec(self.mKeys, self.mNumKeys),
            name: self.mName.into(),
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct NodeAnim {
    pub name: String,
    pub position_keys: Vec<aiVectorKey>,
    pub rotation_keys: Vec<aiQuatKey>,
    pub scaling_keys: Vec<aiVectorKey>,
    pub post_state: u32,
    pub pre_state: u32,
}

impl FromRaw for NodeAnim {}

impl Into<NodeAnim> for &aiNodeAnim {
    fn into(self) -> NodeAnim {
        NodeAnim {
            name: self.mNodeName.into(),
            position_keys: NodeAnim::get_vec(self.mPositionKeys, self.mNumPositionKeys),
            rotation_keys: NodeAnim::get_vec(self.mRotationKeys, self.mNumRotationKeys),
            scaling_keys: NodeAnim::get_vec(self.mScalingKeys, self.mNumScalingKeys),
            post_state: self.mPostState,
            pre_state: self.mPreState,
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct MeshAnim {
    name: String,
    keys: Vec<aiMeshKey>,
}

impl FromRaw for MeshAnim {}

impl Into<MeshAnim> for &aiMeshAnim {
    fn into(self) -> MeshAnim {
        MeshAnim {
            name: self.mName.into(),
            keys: MeshAnim::get_vec(self.mKeys, self.mNumKeys),
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Animation {
    pub name: String,
    pub channels: Vec<NodeAnim>,
    pub duration: f64,
    pub morph_mesh_channels: Vec<MeshMorphAnim>,
    pub mesh_channels: Vec<MeshAnim>,
    pub ticks_per_second: f64,
}

impl FromRaw for Animation {}

impl Into<Animation> for &aiAnimation {
    fn into(self) -> Animation {
        Animation {
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
    let current_directory_buf = get_model("models/3DS/CameraRollAnim.3ds");

    let scene = Scene::from(current_directory_buf.as_str(),
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