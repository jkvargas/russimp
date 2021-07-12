use crate::{sys::*, *};
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct MeshMorphKey {
    pub time: f64,
    pub values: Vec<u32>,
    pub weights: Vec<f64>,
}

impl From<&aiMeshMorphKey> for MeshMorphKey {
    fn from(mesh_morph_key: &aiMeshMorphKey) -> Self {
        Self {
            time: mesh_morph_key.mTime,
            values: utils::get_raw_vec(mesh_morph_key.mValues, mesh_morph_key.mNumValuesAndWeights),
            weights: utils::get_raw_vec(
                mesh_morph_key.mWeights,
                mesh_morph_key.mNumValuesAndWeights,
            ),
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct MeshMorphAnim {
    pub keys: Vec<MeshMorphKey>,
    pub name: String,
}

impl From<&aiMeshMorphAnim> for MeshMorphAnim {
    fn from(mesh: &aiMeshMorphAnim) -> Self {
        Self {
            keys: utils::get_vec(mesh.mKeys, mesh.mNumKeys),
            name: mesh.mName.into(),
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct VectorKey {
    pub time: f64,
    pub value: Vector3D,
}

impl From<&aiVectorKey> for VectorKey {
    fn from(vec: &aiVectorKey) -> Self {
        Self {
            time: vec.mTime,
            value: (&vec.mValue).into(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Derivative)]
#[derivative(Debug)]
pub struct QuatKey {
    pub time: f64,
    pub value: Quaternion,
}

impl From<&aiQuatKey> for QuatKey {
    fn from(quat_key: &aiQuatKey) -> Self {
        Self {
            time: quat_key.mTime,
            value: (&quat_key.mValue).into(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<&aiQuaternion> for Quaternion {
    fn from(quaternion: &aiQuaternion) -> Self {
        Self {
            w: quaternion.w,
            x: quaternion.x,
            y: quaternion.y,
            z: quaternion.z,
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct NodeAnim {
    pub name: String,
    pub position_keys: Vec<VectorKey>,
    pub rotation_keys: Vec<QuatKey>,
    pub scaling_keys: Vec<VectorKey>,
    pub post_state: u32,
    pub pre_state: u32,
}

impl From<&aiNodeAnim> for NodeAnim {
    fn from(node_anim: &aiNodeAnim) -> NodeAnim {
        NodeAnim {
            name: node_anim.mNodeName.into(),
            position_keys: utils::get_vec(node_anim.mPositionKeys, node_anim.mNumPositionKeys),
            rotation_keys: utils::get_vec(node_anim.mRotationKeys, node_anim.mNumRotationKeys),
            scaling_keys: utils::get_vec(node_anim.mScalingKeys, node_anim.mNumScalingKeys),
            post_state: node_anim.mPostState as _,
            pre_state: node_anim.mPreState as _,
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct MeshAnim {
    pub name: String,
    pub keys: Vec<MeshKey>,
}

impl From<&aiMeshAnim> for MeshAnim {
    fn from(mesh: &aiMeshAnim) -> MeshAnim {
        MeshAnim {
            name: mesh.mName.into(),
            keys: utils::get_vec(mesh.mKeys, mesh.mNumKeys),
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct MeshKey {
    pub time: f64,
    pub value: u32,
}

impl From<&aiMeshKey> for MeshKey {
    fn from(mesh_key: &aiMeshKey) -> MeshKey {
        MeshKey {
            time: mesh_key.mTime,
            value: mesh_key.mValue,
        }
    }
}

#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct Animation {
    pub name: String,
    pub channels: Vec<NodeAnim>,
    pub duration: f64,
    pub morph_mesh_channels: Vec<MeshMorphAnim>,
    pub mesh_channels: Vec<MeshAnim>,
    pub ticks_per_second: f64,
}

impl From<&aiAnimation> for Animation {
    fn from(animation: &aiAnimation) -> Self {
        Self {
            name: animation.mName.into(),
            channels: utils::get_vec_from_raw(animation.mChannels, animation.mNumChannels),
            duration: animation.mDuration,
            morph_mesh_channels: utils::get_vec_from_raw(
                animation.mMorphMeshChannels,
                animation.mNumMorphMeshChannels,
            ),
            mesh_channels: utils::get_vec_from_raw(
                animation.mMeshChannels,
                animation.mNumMeshChannels,
            ),
            ticks_per_second: animation.mTicksPerSecond,
        }
    }
}

#[test]
fn camera_roll_animation_read() {
    use crate::scene::{PostProcess, Scene};

    let current_directory_buf = utils::get_model("models/3DS/CameraRollAnim.3ds");

    let scene = Scene::from_file(
        current_directory_buf.as_str(),
        vec![
            PostProcess::CalculateTangentSpace,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::SortByPrimitiveType,
        ],
    )
    .unwrap();

    assert_eq!(1, scene.animations.len());
    assert_eq!("3DSMasterAnim".to_string(), scene.animations[0].name);
    assert_eq!(1, scene.animations[0].channels.len());
    assert_eq!("Camera01".to_string(), scene.animations[0].channels[0].name);

    assert_eq!(0, scene.animations[0].channels[0].pre_state);
    assert_eq!(0, scene.animations[0].channels[0].post_state);
    assert_eq!(0.0, scene.animations[0].channels[0].rotation_keys[0].time);
    assert_eq!(
        0.9999999,
        scene.animations[0].channels[0].rotation_keys[0].value.w
    );
    assert_eq!(
        -0.00046456736,
        scene.animations[0].channels[0].rotation_keys[0].value.x
    );
    assert_eq!(
        0.0,
        scene.animations[0].channels[0].rotation_keys[0].value.y
    );
    assert_eq!(
        0.0,
        scene.animations[0].channels[0].rotation_keys[0].value.z
    );

    assert_eq!(120.0, scene.animations[0].channels[0].rotation_keys[1].time);
    assert_eq!(
        0.7806558,
        scene.animations[0].channels[0].rotation_keys[1].value.w
    );
    assert_eq!(
        -0.6249612,
        scene.animations[0].channels[0].rotation_keys[1].value.x
    );
    assert_eq!(
        0.0,
        scene.animations[0].channels[0].rotation_keys[1].value.y
    );
    assert_eq!(
        0.0,
        scene.animations[0].channels[0].rotation_keys[1].value.z
    );

    assert_eq!(0.0, scene.animations[0].channels[0].scaling_keys[0].time);
    assert_eq!(1.0, scene.animations[0].channels[0].scaling_keys[0].value.x);
    assert_eq!(1.0, scene.animations[0].channels[0].scaling_keys[0].value.y);
    assert_eq!(1.0, scene.animations[0].channels[0].scaling_keys[0].value.z);

    // position keys
    assert_eq!(1, scene.animations[0].channels[0].position_keys.len());
    assert_eq!(0.0, scene.animations[0].channels[0].position_keys[0].time);
    assert_eq!(
        -153.0771,
        scene.animations[0].channels[0].position_keys[0].value.x
    );
    assert_eq!(
        3.272005,
        scene.animations[0].channels[0].position_keys[0].value.y
    );
    assert_eq!(
        22.777624,
        scene.animations[0].channels[0].position_keys[0].value.z
    );

    assert_eq!(120.0, scene.animations[0].duration);
    assert_eq!(0, scene.animations[0].morph_mesh_channels.len());
}

#[test]
fn debug_animations() {
    use crate::scene::{PostProcess, Scene};

    let current_directory_buf = utils::get_model("models/3DS/CameraRollAnim.3ds");

    let scene = Scene::from_file(
        current_directory_buf.as_str(),
        vec![
            PostProcess::CalculateTangentSpace,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::SortByPrimitiveType,
        ],
    )
    .unwrap();

    dbg!(&scene.animations);
}
