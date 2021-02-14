#![crate_name = "russimp"]
#![crate_type = "lib"]
//#![allow(unused_imports, dead_code, unused_variables)]

pub extern crate russimp_sys as sys;

#[cfg(feature = "mint")]
mod impl_mint;
#[cfg(feature = "mint")]
pub use impl_mint::*;

use std::{
    error::Error,
    ffi::IntoStringError,
    fmt,
    fmt::{Display, Formatter},
    str::Utf8Error,
};
use sys::{aiAABB, aiColor3D, aiColor4D, aiMatrix4x4, aiVector2D, aiVector3D};

#[macro_use]
extern crate num_derive;

pub mod animation;
pub mod bone;
pub mod camera;
pub mod face;
pub mod light;
pub mod material;
pub mod mesh;
pub mod metadata;
pub mod node;
pub mod scene;
pub mod texture;

#[derive(Debug)]
pub enum RussimpError {
    Import(String),
    MetadataError(String),
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

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct AABB {
    pub min: Vector3D,
    pub max: Vector3D,
}

impl From<&aiAABB> for AABB {
    fn from(aabb: &aiAABB) -> Self {
        Self {
            max: (&aabb.mMax).into(),
            min: (&aabb.mMin).into(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Color4D {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<&aiColor4D> for Color4D {
    fn from(color: &aiColor4D) -> Self {
        Self {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Color3D {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl From<&aiColor3D> for Color3D {
    fn from(color: &aiColor3D) -> Self {
        Self {
            r: color.r,
            g: color.g,
            b: color.b,
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
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

impl From<&aiMatrix4x4> for Matrix4x4 {
    fn from(matrix: &aiMatrix4x4) -> Self {
        Self {
            a1: matrix.a1,
            a2: matrix.a2,
            a3: matrix.a3,
            a4: matrix.a4,
            b1: matrix.b1,
            b2: matrix.b2,
            b3: matrix.b3,
            b4: matrix.b4,
            c1: matrix.c1,
            c2: matrix.c2,
            c3: matrix.c3,
            c4: matrix.c4,
            d1: matrix.d1,
            d2: matrix.d2,
            d3: matrix.d3,
            d4: matrix.d4,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl From<&aiVector2D> for Vector2D {
    fn from(color: &aiVector2D) -> Self {
        Self {
            x: color.x,
            y: color.y,
        }
    }
}

impl Error for RussimpError {}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<&aiVector3D> for Vector3D {
    fn from(vec: &aiVector3D) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
    }
}

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

/*
impl From<&aiString> for String {
    fn from(string: &aiString) -> Self {
        string.into()
    }
}*/

pub type Russult<T> = Result<T, RussimpError>;

mod utils {
    use std::{cell::RefCell, os::raw::c_uint, ptr::slice_from_raw_parts, rc::Rc};

    #[allow(dead_code)]
    pub(crate) fn get_model(relative_path_from_root: &str) -> String {
        if let Ok(mut github_root) = std::env::var("GITHUB_WORKSPACE") {
            github_root.push('/');
            github_root.push_str(relative_path_from_root);

            github_root
        } else {
            relative_path_from_root.into()
        }
    }

    pub(crate) fn get_raw<'a, TRaw: 'a, TComponent: From<&'a TRaw>>(
        raw: *mut TRaw,
    ) -> Option<TComponent> {
        unsafe { raw.as_ref() }.map(|x| x.into())
    }

    fn _get_rc_raw<TRaw, TComponent>(
        raw: *mut TRaw,
        map: &dyn Fn(&TRaw) -> TComponent,
    ) -> Option<Rc<RefCell<TComponent>>> {
        unsafe { raw.as_ref() }.map(|x| Rc::new(RefCell::new(map(x))))
    }

    pub(crate) fn get_vec<'a, TRaw: 'a, TComponent: From<&'a TRaw>>(
        raw: *mut TRaw,
        len: c_uint,
    ) -> Vec<TComponent> {
        let slice = slice_from_raw_parts(raw as *const TRaw, len as usize);
        if slice.is_null() {
            return vec![];
        }

        let raw = unsafe { slice.as_ref() }.unwrap();
        raw.iter().map(|x| x.into()).collect()
    }

    pub(crate) fn get_raw_vec<TRaw>(raw: *mut TRaw, len: c_uint) -> Vec<TRaw>
    where
        TRaw: Clone,
    {
        let slice = slice_from_raw_parts(raw as *const TRaw, len as usize);
        if slice.is_null() {
            return vec![];
        }

        let raw = unsafe { slice.as_ref() }.unwrap();
        raw.to_vec()
    }

    pub(crate) fn get_vec_from_raw<'a, TComponent: From<&'a TRaw>, TRaw: 'a>(
        raw_source: *mut *mut TRaw,
        num_raw_items: c_uint,
    ) -> Vec<TComponent> {
        let slice = slice_from_raw_parts(raw_source, num_raw_items as usize);
        if slice.is_null() {
            return vec![];
        }

        let raw = unsafe { slice.as_ref() }.unwrap();
        raw.iter()
            .map(|x| (unsafe { x.as_ref() }.unwrap()).into())
            .collect()
    }

    fn _get_vec_rc_from_raw<TComponent, TRaw>(
        raw_source: *mut *mut TRaw,
        num_raw_items: c_uint,
        map: &dyn Fn(&TRaw) -> TComponent,
    ) -> Vec<Rc<RefCell<TComponent>>> {
        let slice = slice_from_raw_parts(raw_source, num_raw_items as usize);
        if slice.is_null() {
            return vec![];
        }

        let raw = unsafe { slice.as_ref() }.unwrap();
        raw.iter()
            .map(|x| Rc::new(RefCell::new(map(unsafe { x.as_ref() }.unwrap()))))
            .collect()
    }

    fn _get_rawvec_from_slice<TRaw>(raw: &[*mut TRaw]) -> Vec<Option<TRaw>>
    where
        TRaw: Clone,
    {
        raw.iter()
            .map(|x| {
                if let Some(raw) = unsafe { x.as_ref() } {
                    Some(raw.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub(crate) fn get_vec_from_slice<'a, TRaw: 'a, TComponent: From<&'a TRaw>>(
        raw: &[*mut TRaw],
    ) -> Vec<Option<TComponent>> {
        raw.iter()
            .map(|x| unsafe { x.as_ref() }.map(|x| x.into()))
            .collect()
    }
}
