use russimp_sys::{aiMesh, aiAnimMesh, aiBone};
use crate::{Vector3d, FromRawVec, RusString};
use crate::bone::Bone;

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

    pub fn get_bitangents(&self) -> Vector3d {
        unsafe { (*self.mesh).mBitangents }.into()
    }

    pub fn get_name(&self) -> String {
        let temp : RusString = unsafe { (*self.mesh).mName }.into();
        temp.into()
    }

    pub fn get_bones(&self) -> Vec<Bone> {
        Self::get_vec(unsafe { (*self.mesh).mBones }, unsafe { (*self.mesh).mNumBones } as usize)
    }
}