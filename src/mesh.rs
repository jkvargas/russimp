use russimp_sys::{aiMesh, aiAnimMesh, aiBone, aiPrimitiveType__aiPrimitiveType_Force32Bit, aiPrimitiveType_aiPrimitiveType_LINE, aiPrimitiveType_aiPrimitiveType_POINT, aiPrimitiveType_aiPrimitiveType_POLYGON, aiPrimitiveType_aiPrimitiveType_TRIANGLE, aiVector3D};
use std::ops::{BitOr, BitAnd};
use crate::{
    FromRawVec,
    bone::Bone,
    face::Face,
    scene::{PostProcessSteps, Scene},
};
use std::ptr::slice_from_raw_parts;
use num_traits::ToPrimitive;

pub struct Mesh<'a> {
    mesh: &'a aiMesh,
    normals: Option<Vec<&'a aiVector3D>>,
    name: String,
    vertices: Vec<&'a aiVector3D>,
    texture_coords: Option<Vec<&'a aiVector3D>>,
    tangents: Option<Vec<&'a aiVector3D>>,
    bitangents: Option<Vec<&'a aiVector3D>>,
    primitive_types: u32,
    uv_components: Vec<u32>,
    material_index: u32,
    method: u32,
}

#[derive(FromPrimitive, Debug, PartialEq, ToPrimitive)]
#[repr(u32)]
pub enum PrimitiveType {
    Force32Bit = aiPrimitiveType__aiPrimitiveType_Force32Bit,
    Line = aiPrimitiveType_aiPrimitiveType_LINE,
    Point = aiPrimitiveType_aiPrimitiveType_POINT,
    Polygon = aiPrimitiveType_aiPrimitiveType_POLYGON,
    Triangle = aiPrimitiveType_aiPrimitiveType_TRIANGLE,
}

impl<'a> FromRawVec for Mesh<'a> {}

impl<'a> Into<Mesh<'a>> for &'a aiMesh {
    fn into(self) -> Mesh<'a> {
        Mesh {
            mesh: self,
            normals: Mesh::get_optional_vec(self.mNormals, self.mNumVertices),
            name: self.mName.into(),
            vertices: Mesh::get_vec(self.mVertices, self.mNumVertices),
            texture_coords: self.mTextureCoords.iter().map(|x| unsafe { x.as_ref() }).collect(),
            tangents: Mesh::get_optional_vec(self.mTangents, self.mNumVertices),
            primitive_types: self.mPrimitiveTypes as u32,
            uv_components: self.mNumUVComponents.to_vec(),
            material_index: self.mMaterialIndex,
            method: self.mMethod,
            bitangents: Mesh::get_optional_vec(self.mBitangents, self.mNumVertices),
        }
    }
}

pub struct AnimMesh {
    anim_mesh: *mut aiAnimMesh
}

impl Into<AnimMesh> for *mut aiAnimMesh {
    fn into(self) -> AnimMesh {
        AnimMesh {
            anim_mesh: self
        }
    }
}

impl BitAnd<PrimitiveType> for PrimitiveType {
    type Output = u32;

    fn bitand(self, rhs: PrimitiveType) -> Self::Output {
        ToPrimitive::to_u32(&self).unwrap() & ToPrimitive::to_u32(&rhs).unwrap()
    }
}

impl BitAnd<PrimitiveType> for u32 {
    type Output = u32;

    fn bitand(self, rhs: PrimitiveType) -> Self::Output {
        self & ToPrimitive::to_u32(&rhs).unwrap()
    }
}

impl BitAnd<u32> for PrimitiveType {
    type Output = u32;

    fn bitand(self, rhs: u32) -> Self::Output {
        ToPrimitive::to_u32(&self).unwrap() & rhs
    }
}

#[test]
fn mesh_available() {
    let current_directory_buf = std::env::current_dir().unwrap().join("russimp-sys/assimp/test/models/BLEND/box.blend");

    let scene = Scene::from(current_directory_buf.to_str().unwrap(),
                            vec![PostProcessSteps::CalcTangentSpace,
                                 PostProcessSteps::Triangulate,
                                 PostProcessSteps::JoinIdenticalVertices,
                                 PostProcessSteps::SortByPType]).unwrap();

    assert_eq!(1, scene.meshes.len());
    assert_eq!(8, scene.meshes[0].normals.as_ref().unwrap().len());
    assert_eq!(8, scene.meshes[0].vertices.len());
    assert!(scene.meshes[0].texture_coords.is_none());
    assert!(scene.meshes[0].tangents.is_none());
    assert_eq!(8, scene.meshes[0].uv_components.len());
    assert_eq!(true, scene.meshes[0].uv_components.iter().all(|x| *x == 0));
    assert_eq!(4, scene.meshes[0].primitive_types);
    assert!(&scene.meshes[0].bitangents.is_none());
}

#[test]
fn bitwise_primitive_types() {
    let current_directory_buf = std::env::current_dir().unwrap().join("russimp-sys/assimp/test/models/BLEND/box.blend");

    let scene = Scene::from(current_directory_buf.to_str().unwrap(),
                            vec![PostProcessSteps::CalcTangentSpace,
                                 PostProcessSteps::Triangulate,
                                 PostProcessSteps::JoinIdenticalVertices,
                                 PostProcessSteps::SortByPType]).unwrap();

    assert_eq!(4, scene.meshes[0].primitive_types & PrimitiveType::Force32Bit);
    assert_eq!(0, scene.meshes[0].primitive_types & PrimitiveType::Line);
    assert_eq!(0, scene.meshes[0].primitive_types & PrimitiveType::Point);
    assert_eq!(4, scene.meshes[0].primitive_types & PrimitiveType::Triangle);
    assert_eq!(0, scene.meshes[0].primitive_types & PrimitiveType::Polygon);
}
