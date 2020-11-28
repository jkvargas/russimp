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
use std::ops::BitAnd;

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

trait FromRawVec {
    fn get_optional_vec<'a, TRaw, TComponent>(raw: *mut TRaw, len: c_uint) -> Option<Vec<TComponent>> where &'a TRaw: Into<TComponent> + 'a {
        let slice = slice_from_raw_parts(raw as *const TRaw, len as usize);
        if slice.is_null() {
            return None;
        }

        let raw = unsafe { slice.as_ref() }.unwrap();
        Some(raw.iter().map(|x| x.into()).collect())
    }

    fn get_vec<'a, TRaw, TComponent>(raw: *mut TRaw, len: c_uint) -> Vec<TComponent> where &'a TRaw: Into<TComponent> + 'a {
        let slice = slice_from_raw_parts(raw as *const TRaw, len as usize);
        let raw = unsafe { slice.as_ref() }.unwrap();
        raw.iter().map(|x| x.into()).collect()
    }

    fn get_vec_from_raw<'a, TComponent, TRaw>(raw_source: *mut *mut TRaw, num_raw_items: c_uint) -> Vec<TComponent> where &'a TRaw: Into<TComponent> + 'a {
        let slice = slice_from_raw_parts(raw_source, num_raw_items as usize);
        let raw = unsafe { slice.as_ref() }.unwrap();
        raw.iter().map(|x| unsafe { x.as_ref() }.unwrap().into()).collect()
    }

    fn get_optional_vec_from_raw<'a, TComponent, TRaw>(raw_source: *mut *mut TRaw, num_raw_items: c_uint) -> Option<Vec<TComponent>> where &'a TRaw: Into<TComponent> + 'a {
        let slice = slice_from_raw_parts(raw_source, num_raw_items as usize);
        if slice.is_null() {
            return None;
        }

        let raw = unsafe { slice.as_ref() }.unwrap();
        Some(raw.iter().map(|x| unsafe { x.as_ref() }.unwrap().into()).collect())
    }
}