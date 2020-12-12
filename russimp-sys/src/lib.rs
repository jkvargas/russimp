#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CStr;

impl Into<String> for aiString {
    fn into(self) -> String {
        let content = unsafe { CStr::from_ptr(self.data.as_ptr()) };
        content.to_str().unwrap().to_string()
    }
}

impl Into<String> for &aiString {
    fn into(self) -> String {
        let content = unsafe { CStr::from_ptr( self.data.as_ptr() )};
        content.to_str().unwrap().to_string()
    }
}