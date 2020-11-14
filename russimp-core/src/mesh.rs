use russimp_sys::{
    aiMesh,
    aiAnimMesh,
    aiBone,
    aiPrimitiveType__aiPrimitiveType_Force32Bit,
    aiPrimitiveType_aiPrimitiveType_LINE,
    aiPrimitiveType_aiPrimitiveType_POINT,
    aiPrimitiveType_aiPrimitiveType_POLYGON,
    aiPrimitiveType_aiPrimitiveType_TRIANGLE};
use crate::{
    face::Face,
    bone::Bone,
    Vector3d,
    FromRawVec,
    Color4d,
};
use std::ops::BitOr;

pub struct Mesh {
    mesh: *mut aiMesh
}

impl Into<Mesh> for *mut aiMesh {
    fn into(self) -> Mesh {
        Mesh {
            mesh: self
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

impl FromRawVec<aiAnimMesh, AnimMesh> for Mesh {}

impl FromRawVec<aiBone, Bone> for Mesh {}

impl Mesh {
    pub fn get_aabb_max(&self) -> Vector3d {
        unsafe { (*self.mesh).mAABB.mMax }.into()
    }

    pub fn get_aabb_min(&self) -> Vector3d {
        unsafe { (*self.mesh).mAABB.mMin }.into()
    }

    pub fn get_anim_meshes(&self) -> Vec<AnimMesh> {
        Self::get_vec(unsafe { (*self.mesh).mAnimMeshes }, unsafe { (*self.mesh).mNumAnimMeshes } as usize)
    }

    pub fn get_bitangents(&self) -> Vec<Vector3d> {
        let res = unsafe { std::slice::from_raw_parts_mut((*self.mesh).mBitangents, (*self.mesh).mNumVertices as usize) };
        res.iter().map(|x| x.into()).collect()
    }

    pub fn get_name(&self) -> String { unsafe { (*self.mesh).mName }.into() }

    pub fn get_bones(&self) -> Vec<Bone> {
        Self::get_vec(unsafe { (*self.mesh).mBones }, unsafe { (*self.mesh).mNumBones } as usize)
    }

    pub fn get_colors(&self) -> Vec<Color4d> {
        unsafe { (*self.mesh).mColors }.to_vec().iter().map(|x| x.into()).collect()
    }

    pub fn get_faces(&self) -> Vec<Face> {
        let res = unsafe { std::slice::from_raw_parts_mut((*self.mesh).mFaces, (*self.mesh).mNumFaces as usize) };
        res.iter().map(|x| x.clone().into()).collect()
    }

    pub fn get_method(&self) -> u32 {
        unsafe { (*self.mesh).mMethod }
    }

    pub fn get_material(&self) -> usize {
        (unsafe { (*self.mesh).mMaterialIndex }) as usize
    }

    pub fn get_normals(&self) -> Vec<Vector3d> {
        let res = unsafe { std::slice::from_raw_parts_mut((*self.mesh).mNormals, (*self.mesh).mNumVertices as usize) };
        res.iter().map(|x| x.into()).collect()
    }

    pub fn get_num_uv_components(&self) -> Vec<u32> {
        unsafe { (*self.mesh).mNumUVComponents }.to_vec()
    }

    pub fn get_primitive_types(&self) -> u32 {
        unsafe { (*self.mesh).mPrimitiveTypes }
    }

    pub fn get_tangents(&self) -> Vec<Vector3d> {
        let res = unsafe { std::slice::from_raw_parts_mut((*self.mesh).mTangents, (*self.mesh).mNumVertices as usize) };
        res.iter().map(|x| x.into()).collect()
    }

    pub fn get_texture_coords(&self) -> Vec<Vector3d> {
        unsafe { (*self.mesh).mTextureCoords }.to_vec().iter().map(|x| x.into()).collect()
    }

    pub fn get_vertices(&self) -> Vec<Vector3d> {
        let res = unsafe { std::slice::from_raw_parts_mut((*self.mesh).mVertices, (*self.mesh).mNumVertices as usize) };
        res.iter().map(|x| x.into()).collect()
    }
}