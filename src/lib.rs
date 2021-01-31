#![crate_name = "russimp"]
#![crate_type = "lib"]
#![allow(unused_imports, dead_code, unused_variables)]

pub extern crate russimp_sys as sys;

use std::{
    error::Error,
    fmt,
    fmt::{Display, Formatter},
    ffi::IntoStringError,
    str::Utf8Error,
    os::raw::c_uint,
    ptr::slice_from_raw_parts,
    rc::Rc,
    cell::RefCell,
};
use sys::aiVector3D;

#[macro_use]
extern crate num_derive;

pub mod bone;
pub mod animation;
pub mod camera;
pub mod face;
pub mod material;
pub mod light;
pub mod scene;
pub mod node;
pub mod metadata;
pub mod mesh;
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

impl Error for RussimpError {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Into<Vector3D> for aiVector3D {
    fn into(self) -> Vector3D {
        Vector3D {
            z: self.z,
            x: self.x,
            y: self.y,
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

pub type Russult<T> = Result<T, RussimpError>;

struct Utils;

impl Utils {
    fn get_model(relative_path_from_root: &str) -> String {
        let mut github_root = std::env::var("GITHUB_WORKSPACE").unwrap();

        github_root.push('/');
        github_root.push_str(relative_path_from_root);

        github_root
    }

    fn get_raw<TRaw, TComponent>(raw: *mut TRaw, map: &dyn Fn(&TRaw) -> TComponent) -> Option<TComponent> {
        unsafe { raw.as_ref() }.map_or(None, |x| Some(map(x)))
    }

    fn get_rc_raw<TRaw, TComponent>(raw: *mut TRaw, map: &dyn Fn(&TRaw) -> TComponent) -> Option<Rc<RefCell<TComponent>>> {
        unsafe { raw.as_ref() }.map_or(None, |x| Some(Rc::new(RefCell::new(map(x)))))
    }

    fn get_vec<TRaw, TComponent>(raw: *mut TRaw, len: c_uint, map: &dyn Fn(&TRaw) -> TComponent) -> Vec<TComponent> {
        let slice = slice_from_raw_parts(raw as *const TRaw, len as usize);
        if slice.is_null() {
            return vec![];
        }

        let raw = unsafe { slice.as_ref() }.unwrap();
        raw.iter().map(|x| map(x)).collect()
    }

    fn get_rawvec<TRaw>(raw: *mut TRaw, len: c_uint) -> Vec<TRaw> where TRaw: Clone {
        let slice = slice_from_raw_parts(raw as *const TRaw, len as usize);
        if slice.is_null() {
            return vec![];
        }

        let raw = unsafe { slice.as_ref() }.unwrap();
        raw.to_vec()
    }

    fn get_vec_from_raw<TComponent, TRaw>(raw_source: *mut *mut TRaw, num_raw_items: c_uint, map: &dyn Fn(&TRaw) -> TComponent) -> Vec<TComponent> {
        let slice = slice_from_raw_parts(raw_source, num_raw_items as usize);
        if slice.is_null() {
            return vec![];
        }

        let raw = unsafe { slice.as_ref() }.unwrap();
        raw.iter().map(|x| map(unsafe { x.as_ref() }.unwrap())).collect()
    }

    fn get_vec_rc_from_raw<TComponent, TRaw>(raw_source: *mut *mut TRaw, num_raw_items: c_uint, map: &dyn Fn(&TRaw) -> TComponent) -> Vec<Rc<RefCell<TComponent>>> {
        let slice = slice_from_raw_parts(raw_source, num_raw_items as usize);
        if slice.is_null() {
            return vec![];
        }

        let raw = unsafe { slice.as_ref() }.unwrap();
        raw.iter().map(|x| Rc::new(RefCell::new(map(unsafe { x.as_ref() }.unwrap())))).collect()
    }

    fn get_rawvec_from_slice<TRaw>(raw: &[*mut TRaw]) -> Vec<Option<TRaw>> where TRaw: Clone {
        raw.iter().map(|x| {
            if let Some(raw) = unsafe { x.as_ref() } {
                Some(raw.clone())
            } else {
                None
            }
        }).collect()
    }

    fn get_vec_from_slice<TRaw, TComponent>(raw: &[*mut TRaw], map: &dyn Fn(&TRaw) -> TComponent) -> Vec<Option<TComponent>> {
        raw.iter().map(|x| {
            if let Some(raw) = unsafe { x.as_ref() } {
                Some(map(raw))
            } else {
                None
            }
        }).collect()
    }
}