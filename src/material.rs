use russimp_sys::{aiMaterial, aiMaterialProperty};
use std::ffi::{CString, CStr};
use std::str::Utf8Error;
use std::os::raw::c_char;
use std::ptr::slice_from_raw_parts;
use crate::scene::{PostProcessSteps, Scene};

pub struct Material<'scene_lifetime> {
    material: &'scene_lifetime aiMaterial,
    properties: Vec<MaterialProperty<'scene_lifetime>>,
}

impl<'scene_lifetime> Drop for Material<'scene_lifetime> {
    fn drop(&mut self) {
        unimplemented!()
    }
}

impl<'scene_lifetime> Into<Material<'scene_lifetime>> for &'scene_lifetime aiMaterial {
    fn into(self) -> Material<'scene_lifetime> {
        let vec_raw: Vec<*mut aiMaterialProperty> = unsafe {
            Vec::from_raw_parts(self.mProperties, self.mNumProperties as usize, self.mNumProperties as usize)
        };

        Material {
            material: self,
            properties: vec_raw.iter().map(|x| unsafe { (*x).as_ref() }.unwrap().into()).collect(),
        }
    }
}

pub struct MaterialProperty<'scene_lifetime> {
    property: &'scene_lifetime aiMaterialProperty,
    data: &'scene_lifetime [u8],
}

impl<'scene_lifetime> Drop for MaterialProperty<'scene_lifetime> {
    fn drop(&mut self) {
        unimplemented!()
    }
}

impl<'scene_lifetime> Into<MaterialProperty<'scene_lifetime>> for &'scene_lifetime aiMaterialProperty {
    fn into(self) -> MaterialProperty<'scene_lifetime> {
        // let slice = slice_from_raw_parts(self.mData as *const u8, self.mDataLength as usize);
        // let data = unsafe { slice.as_ref() }.unwrap();

        MaterialProperty {
            property: self,
            data: &[0x24]
        }
    }
}

// impl<'scene_lifetime> MaterialProperty<'scene_lifetime> {
//
//     pub fn get_data_length(&self) -> u32 { self.property.mDataLength }
//
//     pub fn get_index(&self) -> u32 { self.property.mIndex }
//
//     pub fn get_key(&self) -> String { self.property.mKey.into() }
//
//     pub fn get_semantic(&self) -> u32 { self.property.mSemantic }
//
//     pub fn get_type(&self) -> u32 { self.property.mType }
//}
//
// impl<'scene_lifetime> FromRawVec<aiMaterialProperty, MaterialProperty> for Material<'scene_lifetime> {}
//
// // impl<'scene_lifetime> Material<'scene_lifetime> {
// //     pub fn get_material_properties(&self) -> Vec<MaterialProperty> {
// //         let material_properties_raw: Vec<*mut aiMaterialProperty> = unsafe { Vec::from_raw_parts((*self.material).mProperties, (*self.material).mNumProperties as usize, (*self.material).mNumAllocated as usize) };
// //         material_properties_raw.into_iter().map(|x| x.into()).collect()
// //     }
// // }
//
// #[test]
// fn has_materials() {
//     let current_directory_buf = std::env::current_dir().unwrap().join("russimp-sys/assimp/test/models/BLEND/box.blend");
//
//     dbg!(&current_directory_buf);
//
//     let scene = Scene::from(current_directory_buf.to_str().unwrap(),
//                             vec![PostProcessSteps::CalcTangentSpace,
//                                  PostProcessSteps::Triangulate,
//                                  PostProcessSteps::JoinIdenticalVertices,
//                                  PostProcessSteps::SortByPType]).unwrap();
//
//     assert_eq!(1, scene.materials.len());
//     assert_eq!(41, scene.materials[0].properties.len());
//
//     //dbg!(scene.materials[0].properties[0].data);
//
// }