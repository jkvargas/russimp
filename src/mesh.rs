use std::ops::BitAnd;

use crate::{
    sys::{
        aiVector3D,
        aiMesh,
        aiColor4D,
        aiAABB,
        aiPrimitiveType__aiPrimitiveType_Force32Bit,
        aiPrimitiveType_aiPrimitiveType_LINE,
        aiPrimitiveType_aiPrimitiveType_POINT,
        aiPrimitiveType_aiPrimitiveType_POLYGON,
        aiPrimitiveType_aiPrimitiveType_TRIANGLE,
        aiAnimMesh,
    },
    FromRaw,
    bone::Bone,
    face::Face,
    scene::{
        PostProcessSteps,
        Scene,
    },
    get_model};

use num_traits::ToPrimitive;

use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Mesh {
    pub normals: Vec<aiVector3D>,
    pub name: String,
    pub vertices: Vec<aiVector3D>,
    pub texture_coords: Vec<Option<aiVector3D>>,
    pub tangents: Vec<aiVector3D>,
    pub bitangents: Vec<aiVector3D>,
    pub uv_components: Vec<u32>,
    pub primitive_types: u32,
    pub bones: Vec<Bone>,
    pub material_index: u32,
    pub method: u32,
    pub anim_meshes: Vec<AnimMesh>,
    pub faces: Vec<Face>,
    pub colors: Vec<Option<aiColor4D>>,
    pub aabb: aiAABB,
}

#[derive(Derivative, FromPrimitive, PartialEq, ToPrimitive)]
#[derivative(Debug)]
#[repr(u32)]
pub enum PrimitiveType {
    Force32Bit = aiPrimitiveType__aiPrimitiveType_Force32Bit,
    Line = aiPrimitiveType_aiPrimitiveType_LINE,
    Point = aiPrimitiveType_aiPrimitiveType_POINT,
    Polygon = aiPrimitiveType_aiPrimitiveType_POLYGON,
    Triangle = aiPrimitiveType_aiPrimitiveType_TRIANGLE,
}

impl FromRaw for Mesh {}

impl Into<Mesh> for &aiMesh {
    fn into(self) -> Mesh {
        Mesh {
            normals: Mesh::get_vec(self.mNormals, self.mNumVertices),
            name: self.mName.into(),
            vertices: Mesh::get_vec(self.mVertices, self.mNumVertices),
            texture_coords: Mesh::get_rawvec_from_slice(&self.mTextureCoords),
            tangents: Mesh::get_vec(self.mTangents, self.mNumVertices),
            bitangents: Mesh::get_vec(self.mBitangents, self.mNumVertices),
            uv_components: self.mNumUVComponents.to_vec(),
            primitive_types: self.mPrimitiveTypes as u32,
            bones: Mesh::get_vec_from_raw(self.mBones, self.mNumBones),
            material_index: self.mMaterialIndex,
            method: self.mMethod,
            anim_meshes: Mesh::get_vec_from_raw(self.mAnimMeshes, self.mNumAnimMeshes),
            faces: Mesh::get_vec(self.mFaces, self.mNumFaces),
            colors: Mesh::get_rawvec_from_slice(&self.mColors),
            aabb: self.mAABB,
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct AnimMesh {
    bitangents: Vec<aiVector3D>,
}

impl FromRaw for AnimMesh {}

impl Into<AnimMesh> for &aiAnimMesh {
    fn into(self) -> AnimMesh {
        AnimMesh {
            bitangents: Mesh::get_vec(self.mBitangents, self.mNumVertices),
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
    let current_directory_buf = get_model("models/BLEND/box.blend");

    let scene = Scene::from(current_directory_buf.as_str(),
                            vec![PostProcessSteps::CalcTangentSpace,
                                 PostProcessSteps::Triangulate,
                                 PostProcessSteps::JoinIdenticalVertices,
                                 PostProcessSteps::SortByPType]).unwrap();

    assert_eq!(1, scene.meshes.len());
    assert_eq!(8, scene.meshes[0].normals.len());
    assert_eq!(8, scene.meshes[0].vertices.len());
    assert!(scene.meshes[0].texture_coords.iter().all(|x| x.is_none()));
    assert!(scene.meshes[0].tangents.is_empty());
    assert!(scene.meshes[0].bitangents.is_empty());
    assert_eq!(8, scene.meshes[0].uv_components.len());
    assert_eq!(true, scene.meshes[0].uv_components.iter().all(|x| *x == 0));
    assert_eq!(4, scene.meshes[0].primitive_types);
    assert!(scene.meshes[0].bones.is_empty());
    assert!(scene.meshes[0].anim_meshes.is_empty());
    assert_eq!(12, scene.meshes[0].faces.len());
    assert!(&scene.meshes[0].anim_meshes.is_empty());
    assert_eq!(0, scene.meshes[0].method);
    assert_eq!(0, scene.meshes[0].material_index);
    assert_eq!(0.0, scene.meshes[0].aabb.mMin.x);
    assert_eq!(0.0, scene.meshes[0].aabb.mMin.y);
    assert_eq!(0.0, scene.meshes[0].aabb.mMin.z);
    assert_eq!(0.0, scene.meshes[0].aabb.mMax.x);
    assert_eq!(0.0, scene.meshes[0].aabb.mMax.y);
    assert_eq!(0.0, scene.meshes[0].aabb.mMax.z);
    assert!(scene.meshes[0].colors.iter().all(|x| x.is_none()));
}

#[test]
fn bitwise_primitive_types() {
    let current_directory_buf = get_model("models/BLEND/box.blend");

    let scene = Scene::from(current_directory_buf.as_str(),
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
