use crate::{sys::aiCamera, Vector3D};
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Camera {
    pub name: String,
    pub aspect: f32,
    pub clip_plane_far: f32,
    pub clip_plane_near: f32,
    pub horizontal_fov: f32,
    pub look_at: Vector3D,
    pub position: Vector3D,
    pub up: Vector3D,
}

impl From<&aiCamera> for Camera {
    fn from(camera: &aiCamera) -> Self {
        Self {
            name: camera.mName.into(),
            aspect: camera.mAspect,
            clip_plane_far: camera.mClipPlaneFar,
            clip_plane_near: camera.mClipPlaneNear,
            horizontal_fov: camera.mHorizontalFOV,
            look_at: (&camera.mLookAt).into(),
            position: (&camera.mPosition).into(),
            up: (&camera.mUp).into(),
        }
    }
}

#[test]
fn camera_available() {
    use crate::{
        scene::{PostProcess, Scene},
        utils,
    };

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

#[test]
fn debug_camera() {
    use crate::{
        scene::{PostProcess, Scene},
        utils,
    };

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

    dbg!(&scene.cameras);
}
