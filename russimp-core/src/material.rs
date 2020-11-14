use russimp_sys::{aiMaterial, aiMaterialProperty};
use std::ffi::CString;
use crate::{RussimpError, RusString};

pub struct Material {
    material: *mut aiMaterial
}

impl Into<Material> for *mut aiMaterial {
    fn into(self) -> Material {
        Material {
            material: self
        }
    }
}

pub struct MaterialProperty {
    property: *mut aiMaterialProperty
}

impl Into<MaterialProperty> for *mut aiMaterialProperty {
    fn into(self) -> MaterialProperty {
        MaterialProperty {
            property: self
        }
    }
}

impl MaterialProperty {
    pub fn get_data(&self) -> Result<String, RussimpError> {
        let c_string = unsafe { CString::from_raw((*self.property).mData) };

        match c_string.into_string() {
            Ok(result) => Ok(result),
            Err(err) => Err(RussimpError::MeterialError(err.to_string()))
        }
    }

    pub fn get_data_length(&self) -> u32 {
        unsafe { (*self.property).mDataLength }
    }

    pub fn get_index(&self) -> u32 {
        unsafe { (*self.property).mIndex }
    }

    pub fn get_key(&self) -> String {
        let content: RusString = unsafe { (*self.property).mKey.into() };
        content.into()
    }

    pub fn get_semantic(&self) -> u32 {
        unsafe { (*self.property).mSemantic }
    }

    pub fn get_type(&self) -> u32 {
        unsafe {
            (*self.property).mType
        }
    }
}

impl Material {
    pub fn get_material_properties(&self) -> Vec<MaterialProperty> {
        let material_properties_raw: Vec<*mut aiMaterialProperty> = unsafe { Vec::from_raw_parts((*self.material).mProperties, (*self.material).mNumProperties as usize, (*self.material).mNumAllocated as usize) };
        material_properties_raw.into_iter().map(|x| x.into()).collect()
    }
}