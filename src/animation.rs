use crate::{
    scene::{PostProcessSteps, Scene},
    sys::{
        aiAnimation, aiMesh, aiMeshAnim, aiMeshKey, aiMeshMorphAnim, aiMeshMorphKey, aiNode,
        aiNodeAnim, aiQuatKey, aiQuaternion, aiVector3D, aiVectorKey,
    },
    Utils, Vector3D,
};

use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct MeshMorphKey {
    pub time: f64,
    pub values: Vec<u32>,
    pub weights: Vec<f64>,
}

impl MeshMorphKey {
    pub fn new(mesh_morph_key: &aiMeshMorphKey) -> MeshMorphKey {
        Self {
            time: mesh_morph_key.mTime,
            values: Utils::get_rawvec(mesh_morph_key.mValues, mesh_morph_key.mNumValuesAndWeights),
            weights: Utils::get_rawvec(
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

impl MeshMorphAnim {
    pub fn new(mesh: &aiMeshMorphAnim) -> MeshMorphAnim {
        MeshMorphAnim {
            keys: Utils::get_vec(mesh.mKeys, mesh.mNumKeys, &MeshMorphKey::new),
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

impl VectorKey {
    pub fn new(vec: &aiVectorKey) -> VectorKey {
        Self {
            time: vec.mTime,
            value: Vector3D::new(&vec.mValue),
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

impl QuatKey {
    pub fn new(quat_key: &aiQuatKey) -> QuatKey {
        Self {
            time: quat_key.mTime,
            value: Quaternion::new(&quat_key.mValue),
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

impl Quaternion {
    pub fn new(quaternion: &aiQuaternion) -> Quaternion {
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

impl NodeAnim {
    pub fn new(node_anim: &aiNodeAnim) -> NodeAnim {
        NodeAnim {
            name: node_anim.mNodeName.into(),
            position_keys: Utils::get_vec(
                node_anim.mPositionKeys,
                node_anim.mNumPositionKeys,
                &VectorKey::new,
            ),
            rotation_keys: Utils::get_vec(
                node_anim.mRotationKeys,
                node_anim.mNumRotationKeys,
                &QuatKey::new,
            ),
            scaling_keys: Utils::get_vec(
                node_anim.mScalingKeys,
                node_anim.mNumScalingKeys,
                &VectorKey::new,
            ),
            post_state: node_anim.mPostState,
            pre_state: node_anim.mPreState,
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct MeshAnim {
    name: String,
    keys: Vec<MeshKey>,
}

impl MeshAnim {
    pub fn new(mesh: &aiMeshAnim) -> MeshAnim {
        MeshAnim {
            name: mesh.mName.into(),
            keys: Utils::get_vec(mesh.mKeys, mesh.mNumKeys, &MeshKey::new),
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
struct MeshKey {
    time: f64,
    value: u32,
}

impl MeshKey {
    pub fn new(mesh_key: &aiMeshKey) -> MeshKey {
        MeshKey {
            time: mesh_key.mTime,
            value: mesh_key.mValue,
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

impl Animation {
    pub fn new(animation: &aiAnimation) -> Animation {
        Self {
            name: animation.mName.into(),
            channels: Utils::get_vec_from_raw(
                animation.mChannels,
                animation.mNumChannels,
                &NodeAnim::new,
            ),
            duration: animation.mDuration,
            morph_mesh_channels: Utils::get_vec_from_raw(
                animation.mMorphMeshChannels,
                animation.mNumMorphMeshChannels,
                &MeshMorphAnim::new,
            ),
            mesh_channels: Utils::get_vec_from_raw(
                animation.mMeshChannels,
                animation.mNumMeshChannels,
                &MeshAnim::new,
            ),
            ticks_per_second: animation.mTicksPerSecond,
        }
    }
}

#[test]
fn camera_roll_animation_read() {
    let current_directory_buf = Utils::get_model("models/3DS/CameraRollAnim.3ds");

    let scene = Scene::from(
        current_directory_buf.as_str(),
        vec![
            PostProcessSteps::CalculateTangentSpace,
            PostProcessSteps::Triangulate,
            PostProcessSteps::JoinIdenticalVertices,
            PostProcessSteps::SortByPrimitiveType,
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
    let current_directory_buf = Utils::get_model("models/3DS/CameraRollAnim.3ds");

    let scene = Scene::from(
        current_directory_buf.as_str(),
        vec![
            PostProcessSteps::CalculateTangentSpace,
            PostProcessSteps::Triangulate,
            PostProcessSteps::JoinIdenticalVertices,
            PostProcessSteps::SortByPrimitiveType,
        ],
    )
    .unwrap();

    dbg!(&scene.animations);
}
