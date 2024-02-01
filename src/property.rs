use std::ffi::CString;

use russimp_sys::{
    aiCreatePropertyStore, aiMatrix4x4, aiPropertyStore, aiReleasePropertyStore,
    aiSetImportPropertyFloat, aiSetImportPropertyInteger, aiSetImportPropertyMatrix,
    aiSetImportPropertyString, aiString,
};

pub enum Property {
    String(&'static str),
    Float(f32),
    Integer(i32),
    Matrix([[f32; 4]; 4]),
}

pub struct PropertyStore {
    ptr: *mut aiPropertyStore,
}

impl Drop for PropertyStore {
    #[inline]
    fn drop(&mut self) {
        unsafe { aiReleasePropertyStore(self.ptr) };
    }
}

impl Default for PropertyStore {
    fn default() -> Self {
        let ptr = unsafe { aiCreatePropertyStore() };
        Self { ptr }
    }
}

impl PropertyStore {
    pub fn set_integer(&mut self, name: &str, value: i32) {
        let c_name = CString::new(name).unwrap();
        unsafe { aiSetImportPropertyInteger(self.ptr, c_name.as_ptr(), value) };
    }

    pub fn set_float(&mut self, name: &str, value: f32) {
        let c_name = CString::new(name).unwrap();
        unsafe { aiSetImportPropertyFloat(self.ptr, c_name.as_ptr(), value) };
    }

    pub fn set_string(&mut self, name: &str, value: &str) {
        let c_name = CString::new(name).unwrap();
        let bytes: &[::std::os::raw::c_char] = unsafe { std::mem::transmute(value.as_bytes()) };
        let mut string = aiString {
            length: bytes.len() as u32,
            data: [0; 1024],
        };
        let n = std::cmp::min(string.data.len(), bytes.len());
        string.data[0..n].copy_from_slice(&bytes[0..n]);
        unsafe { aiSetImportPropertyString(self.ptr, c_name.as_ptr(), &string as *const aiString) };
    }

    pub fn set_matrix(&mut self, name: &str, value: [[f32; 4]; 4]) {
        let c_name = CString::new(name).unwrap();
        // NOTE: Assuming column-major matrix
        let matrix = aiMatrix4x4 {
            a1: value[0][0],
            a2: value[1][0],
            a3: value[2][0],
            a4: value[3][0],
            b1: value[0][1],
            b2: value[1][1],
            b3: value[2][1],
            b4: value[3][1],
            c1: value[0][2],
            c2: value[1][2],
            c3: value[2][2],
            c4: value[3][2],
            d1: value[0][3],
            d2: value[1][3],
            d3: value[2][3],
            d4: value[3][3],
        };
        unsafe {
            aiSetImportPropertyMatrix(self.ptr, c_name.as_ptr(), &matrix as *const aiMatrix4x4)
        };
    }

    pub(crate) fn as_ptr(&self) -> *mut aiPropertyStore {
        self.ptr
    }
}

impl<T: Iterator<Item = (&'static str, Property)>> From<T> for PropertyStore {
    fn from(value: T) -> Self {
        let mut props = Self::default();
        for (name, prop) in value {
            match prop {
                Property::String(v) => props.set_string(name, v),
                Property::Float(v) => props.set_float(name, v),
                Property::Integer(v) => props.set_integer(name, v),
                Property::Matrix(v) => props.set_matrix(name, v),
            }
        }
        props
    }
}
