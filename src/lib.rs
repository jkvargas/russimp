#![allow(unused_imports, dead_code, unused_variables)]

#![crate_name = "russimp"]
#![crate_type = "lib"]

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
mod texture;

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

trait FromRaw {
    fn get_raw<'a, TRaw, TComponent>(raw: *mut TRaw) -> Option<TComponent> where &'a TRaw: Into<TComponent> + 'a {
        unsafe { raw.as_ref() }.map_or(None, |x| Some(x.into()))
    }

    fn get_rc_raw<'a, TRaw, TComponent>(raw: *mut TRaw) -> Option<Rc<RefCell<TComponent>>> where &'a TRaw: Into<TComponent> + 'a {
        unsafe { raw.as_ref() }.map_or(None, |x| Some(Rc::new(RefCell::new(x.into()))))
    }

    fn get_vec<'a, TRaw, TComponent>(raw: *mut TRaw, len: c_uint) -> Vec<TComponent> where &'a TRaw: Into<TComponent> + 'a {
        let slice = slice_from_raw_parts(raw as *const TRaw, len as usize);
        if slice.is_null() {
            return vec![];
        }

        let raw = unsafe { slice.as_ref() }.unwrap();
        raw.iter().map(|x| x.into()).collect()
    }

    fn get_rawvec<'a, TRaw>(raw: *mut TRaw, len: c_uint) -> Vec<&'a TRaw> {
        let slice = slice_from_raw_parts(raw as *const TRaw, len as usize);
        if slice.is_null() {
            return vec![];
        }

        let raw = unsafe { slice.as_ref() }.unwrap();
        raw.iter().collect()
    }

    fn get_vec_from_raw<'a, TComponent, TRaw>(raw_source: *mut *mut TRaw, num_raw_items: c_uint) -> Vec<TComponent> where &'a TRaw: Into<TComponent> + 'a {
        let slice = slice_from_raw_parts(raw_source, num_raw_items as usize);
        if slice.is_null() {
            return vec![];
        }

        let raw = unsafe { slice.as_ref() }.unwrap();
        raw.iter().map(|x| unsafe { x.as_ref() }.unwrap().into()).collect()
    }

    fn get_vec_rc_from_raw<'a, TComponent, TRaw>(raw_source: *mut *mut TRaw, num_raw_items: c_uint) -> Vec<Rc<RefCell<TComponent>>> where &'a TRaw: Into<TComponent> + 'a {
        let slice = slice_from_raw_parts(raw_source, num_raw_items as usize);
        if slice.is_null() {
            return vec![];
        }

        let raw = unsafe { slice.as_ref() }.unwrap();
        raw.iter().map(|x| Rc::new(RefCell::new(unsafe { x.as_ref() }.unwrap().into()))).collect()
    }

    fn get_rawvec_from_slice<'a, TRaw>(raw: &[*mut TRaw]) -> Vec<Option<&'a TRaw>> {
        raw.iter().map(|x| {
            if let Some(raw) = unsafe { x.as_ref() } {
                Some(raw)
            } else {
                None
            }
        }).collect()
    }

    fn get_vec_from_slice<'a, TRaw, TComponent>(raw: &[*mut TRaw]) -> Vec<Option<TComponent>> where &'a TRaw: Into<TComponent> + 'a {
        raw.iter().map(|x| {
            if let Some(raw) = unsafe { x.as_ref() } {
                Some(raw.into())
            } else {
                None
            }
        }).collect()
    }
}