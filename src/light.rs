use russimp_sys::aiLight;

pub struct Light {
    light: *mut aiLight
}

impl Into<Light> for *mut aiLight {
    fn into(self) -> Light {
        Light {
            light: self
        }
    }
}