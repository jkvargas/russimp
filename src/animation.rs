use russimp_sys::{aiAnimation, aiNodeAnim, aiVectorKey, aiVector3D};
use crate::scene::{Scene, PostProcessSteps};
use crate::FromRaw;

pub struct Animation<'a> {
    animation: &'a aiAnimation,
    pub name: String,
    pub channels: Vec<NodeAnim<'a>>,
}

pub struct NodeAnim<'a> {
    anim_node: &'a aiNodeAnim,
    name: String,
    position_keys: Vec<&'a aiVectorKey>,
}

impl<'a> Into<NodeAnim<'a>> for &'a aiNodeAnim {
    fn into(self) -> NodeAnim<'a> {
        NodeAnim {
            anim_node: self,
            name: self.mNodeName.into(),
            position_keys: NodeAnim::get_vec(self.mPositionKeys, self.mNumPositionKeys),
        }
    }
}

impl<'a> FromRaw for Animation<'a> {}

impl<'a> FromRaw for NodeAnim<'a> {}

impl<'a> Into<Animation<'a>> for &'a aiAnimation {
    fn into(self) -> Animation<'a> {
        Animation {
            animation: self,
            name: self.mName.into(),
            channels: Animation::get_vec_from_raw(self.mChannels, self.mNumChannels),
        }
    }
}

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

    // position keys
    assert_eq!(1, scene.animations[0].channels[0].position_keys.len());
    assert_eq!(0.0, scene.animations[0].channels[0].position_keys[0].mTime);
    assert_eq!(-153.0771, scene.animations[0].channels[0].position_keys[0].mValue.x);
    assert_eq!(3.272005, scene.animations[0].channels[0].position_keys[0].mValue.y);
    assert_eq!(22.777624, scene.animations[0].channels[0].position_keys[0].mValue.z);
}