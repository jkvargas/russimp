use russimp_sys::{aiMesh, aiAnimMesh, aiBone, aiPrimitiveType__aiPrimitiveType_Force32Bit, aiPrimitiveType_aiPrimitiveType_LINE, aiPrimitiveType_aiPrimitiveType_POINT, aiPrimitiveType_aiPrimitiveType_POLYGON, aiPrimitiveType_aiPrimitiveType_TRIANGLE, aiVector3D};
use std::ops::BitOr;
use crate::FromRawVec;
use crate::bone::Bone;
use crate::face::Face;
use crate::scene::{PostProcessSteps, Scene};
use std::ptr::slice_from_raw_parts;

pub struct Mesh<'a> {
    mesh: &'a aiMesh,
    normals: Vec<&'a aiVector3D>,
    name: String,
    vertices: Vec<&'a aiVector3D>,
}

impl<'a> FromRawVec for Mesh<'a> {}

impl<'a> Into<Mesh<'a>> for &'a aiMesh {
    fn into(self) -> Mesh<'a> {
        Mesh {
            mesh: self,
            normals: Mesh::get_vec_from_raw_mut(self.mNormals, self.mNumVertices),
            name: self.mName.into(),
            vertices: Mesh::get_vec_from_raw_mut(self.mVertices, self.mNumVertices),
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

#[repr(u32)]
pub enum PrimitiveType {
    Force32Bit = aiPrimitiveType__aiPrimitiveType_Force32Bit,
    Line = aiPrimitiveType_aiPrimitiveType_LINE,
    Point = aiPrimitiveType_aiPrimitiveType_POINT,
    Polygon = aiPrimitiveType_aiPrimitiveType_POLYGON,
    Triangle = aiPrimitiveType_aiPrimitiveType_TRIANGLE,
}

impl BitOr<PrimitiveType> for PrimitiveType {
    type Output = u32;

    fn bitor(self, rhs: PrimitiveType) -> Self::Output {
        self | rhs
    }
}

impl BitOr<u32> for PrimitiveType {
    type Output = ();

    fn bitor(self, rhs: u32) -> Self::Output {
        self | rhs
    }
}

impl BitOr<PrimitiveType> for u32 {
    type Output = u32;

    fn bitor(self, rhs: PrimitiveType) -> Self::Output {
        self | rhs
    }
}


#[test]
pub fn mesh_available() {
    let current_directory_buf = std::env::current_dir().unwrap().join("russimp-sys/assimp/test/models/BLEND/box.blend");

    let scene = Scene::from(current_directory_buf.to_str().unwrap(),
                            vec![PostProcessSteps::CalcTangentSpace,
                                 PostProcessSteps::Triangulate,
                                 PostProcessSteps::JoinIdenticalVertices,
                                 PostProcessSteps::SortByPType]).unwrap();

    assert_eq!(1, scene.meshes.len());
    assert_eq!(8, scene.meshes[0].normals.len());
    assert_eq!(8, scene.meshes[0].vertices.len());
}
