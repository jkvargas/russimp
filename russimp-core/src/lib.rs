use std::{
    error::Error,
    fmt,
    fmt::{Display, Formatter},
    ffi::{CStr, IntoStringError},
    str::Utf8Error,
};
use russimp_sys::{aiString, aiVector3D, aiColor4D};

mod material;
mod scene;
mod mesh;
mod bone;
mod animation;
mod camera;
mod light;
mod metadata;
mod node;
mod face;


#[derive(Debug)]
pub enum RussimpError {
    Import(String),
    MeterialError(String),
    Primitive(String),
}

impl Display for RussimpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RussimpError::Import(content) => {
                return write!(f, "{}", content);
            }
            _ => {
                return write!(f, "unknown error");
            }
        }
    }
}

impl Error for RussimpError {}

impl Into<RussimpError> for Utf8Error {
    fn into(self) -> RussimpError {
        RussimpError::Primitive(self.to_string())
    }
}

impl Into<RussimpError> for IntoStringError {
    fn into(self) -> RussimpError {
        RussimpError::Primitive(self.to_string())
    }
}

pub type Russult<T> = Result<T, RussimpError>;

pub struct Vector3d(f32, f32, f32);

pub struct Color4d(f32, f32, f32, f32);

impl Into<Color4d> for &*mut aiColor4D {
    fn into(self) -> Color4d {
        Color4d(unsafe { (*(*self)).r }, unsafe { (*(*self)).g }, unsafe { (*(*self)).b }, unsafe { (*(*self)).a })
    }
}

impl Into<Vector3d> for *mut aiVector3D {
    fn into(self) -> Vector3d {
        Vector3d(unsafe { (*self).x }, unsafe { (*self).y }, unsafe { (*self).z })
    }
}

impl Into<Vector3d> for &aiVector3D {
    fn into(self) -> Vector3d {
        Vector3d(self.x, self.y, self.z)
    }
}

impl Into<Vector3d> for aiVector3D {
    fn into(self) -> Vector3d {
        Vector3d(self.x, self.y, self.z)
    }
}

impl Into<Vector3d> for &*mut aiVector3D {
    fn into(self) -> Vector3d {
        Vector3d(unsafe { (*(*self)).x }, unsafe { (*(*self)).y }, unsafe { (*(*self)).z })
    }
}

trait FromRawVec<TRawType, TResultType> where *mut TRawType: Into<TResultType> {
    fn get_vec(raw: *mut *mut TRawType, length: usize) -> Vec<TResultType> {
        let vec_raw: Vec<*mut TRawType> = unsafe { Vec::from_raw_parts(raw, length, length) };
        vec_raw.into_iter().map(|x| x.into()).collect()
    }
}