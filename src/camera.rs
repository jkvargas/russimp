use russimp_sys::{
    aiCamera,
    aiVector3D,
};

use crate::{
    FromRaw,
    scene::{
        PostProcessSteps,
        Scene,
    },
};

pub struct Camera<'a> {
    camera: &'a aiCamera,
    pub name: String,
    pub aspect: f32,
    pub clip_plane_far: f32,
    pub clip_plane_near: f32,
    pub horizontal_fov: f32,
    pub look_at: aiVector3D,
    pub position: aiVector3D,
    pub up: aiVector3D,
}

impl<'a> FromRaw for Camera<'a> {}

impl<'a> Into<Camera<'a>> for &'a aiCamera {
    fn into(self) -> Camera<'a> {
        Camera {
            camera: self,
            name: self.mName.into(),
            aspect: self.mAspect,
            clip_plane_far: self.mClipPlaneFar,
            clip_plane_near: self.mClipPlaneNear,
            horizontal_fov: self.mHorizontalFOV,
            look_at: self.mLookAt,
            position: self.mPosition,
            up: self.mUp,
        }
    }
}

#[test]
fn camera_available() {
    let current_directory_buf = std::env::current_dir().unwrap().join("russimp-sys/assimp/test/models/3DS/CameraRollAnim.3ds");

    let scene = Scene::from(current_directory_buf.to_str().unwrap(),
                            vec![PostProcessSteps::CalcTangentSpace,
                                 PostProcessSteps::Triangulate,
                                 PostProcessSteps::JoinIdenticalVertices,
                                 PostProcessSteps::SortByPType]).unwrap();

    assert_eq!(1, scene.cameras.len());

    assert_eq!(-153.0771, scene.cameras[0].position.x);
    assert_eq!(3.272005, scene.cameras[0].position.y);
    assert_eq!(22.777624, scene.cameras[0].position.z);

    assert_eq!(0.0, scene.cameras[0].look_at.x);
    assert_eq!(0.0, scene.cameras[0].look_at.y);
    assert_eq!(1.0, scene.cameras[0].look_at.z);

    assert_eq!(0.0, scene.cameras[0].up.x);
    assert_eq!(1.0, scene.cameras[0].up.y);
    assert_eq!(0.0, scene.cameras[0].up.z);

    assert_eq!(0.9308422, scene.cameras[0].horizontal_fov);
    assert_eq!(0.0, scene.cameras[0].clip_plane_near);
    assert_eq!(1000.0, scene.cameras[0].clip_plane_far);
    assert_eq!("Camera01".to_string(), scene.cameras[0].name);
}