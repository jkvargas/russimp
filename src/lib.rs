use std::{
    error::Error,
    fmt,
    fmt::{Display, Formatter},
    ffi::{CStr, IntoStringError},
    str::Utf8Error,
};
use russimp_sys::{aiVector3D, aiColor4D, aiMatrix4x4};
use std::os::raw::c_uint;
use std::ptr::slice_from_raw_parts;

#[macro_use]
extern crate num_derive;

mod bone;
mod animation;
mod camera;
mod face;
mod material;
mod light;
mod scene;
mod node;
mod metadata;
mod mesh;


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

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Color4d {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Matrix4x4 {
    pub a1: f32,
    pub a2: f32,
    pub a3: f32,
    pub a4: f32,
    pub b1: f32,
    pub b2: f32,
    pub b3: f32,
    pub b4: f32,
    pub c1: f32,
    pub c2: f32,
    pub c3: f32,
    pub c4: f32,
    pub d1: f32,
    pub d2: f32,
    pub d3: f32,
    pub d4: f32,
}

impl Into<Matrix4x4> for aiMatrix4x4 {
    fn into(self) -> Matrix4x4 {
        Matrix4x4 {
            a1: self.a1,
            a2: self.a2,
            a3: self.a3,
            a4: self.a4,
            b1: self.b1,
            b2: self.b2,
            b3: self.b3,
            b4: self.b4,
            c1: self.c1,
            c2: self.c2,
            c3: self.c3,
            c4: self.c4,
            d1: self.d1,
            d2: self.d2,
            d3: self.d3,
            d4: self.d4,
        }
    }
}

impl Into<Color4d> for &*mut aiColor4D {
    fn into(self) -> Color4d {
        Color4d {
            r: unsafe { (*(*self)).r },
            g: unsafe { (*(*self)).g },
            b: unsafe { (*(*self)).b },
            a: unsafe { (*(*self)).a },
        }
    }
}

impl Into<Vector3d> for *mut aiVector3D {
    fn into(self) -> Vector3d {
        Vector3d {
            x: unsafe { (*self).x },
            y: unsafe { (*self).y },
            z: unsafe { (*self).z },
        }
    }
}

impl Into<Vector3d> for &aiVector3D {
    fn into(self) -> Vector3d {
        Vector3d { x: self.x, y: self.y, z: self.z }
    }
}

impl Into<Vector3d> for aiVector3D {
    fn into(self) -> Vector3d {
        Vector3d { x: self.x, y: self.y, z: self.z }
    }
}

impl Into<Vector3d> for &*mut aiVector3D {
    fn into(self) -> Vector3d {
        Vector3d {
            x: unsafe { (*(*self)).x },
            y: unsafe { (*(*self)).y },
            z: unsafe { (*(*self)).z },
        }
    }
}