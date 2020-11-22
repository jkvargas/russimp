use russimp_sys::aiCamera;

pub struct Camera {
    camera: *mut aiCamera
}

impl Into<Camera> for *mut aiCamera {
    fn into(self) -> Camera {
        Camera {
            camera: self
        }
    }
}