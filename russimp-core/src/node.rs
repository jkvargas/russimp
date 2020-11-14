use russimp_sys::aiNode;
use crate::metadata::MetaData;
use crate::{FromRawVec, Matrix4x4};

pub struct Node {
    node: *mut aiNode
}

impl Into<Node> for *mut aiNode {
    fn into(self) -> Node {
        Node {
            node: self
        }
    }
}

impl FromRawVec<aiNode, Node> for Node {}

impl Node {
    pub fn get_name(&self) -> String { unsafe { (*self.node).mName }.into() }

    pub fn get_metadata(&self) -> MetaData {
        unsafe { (*self.node).mMetaData }.into()
    }

    pub fn get_meshes(&self) -> Vec<u32> {
        let res = unsafe { std::slice::from_raw_parts_mut((*self.node).mMeshes, (*self.node).mNumMeshes as usize) };
        res.to_vec()
    }

    pub fn get_children(&self) -> Vec<Node> {
        Self::get_vec(unsafe { (*self.node).mChildren }, unsafe { (*self.node).mNumChildren } as usize)
    }

    pub fn get_parent(&self) -> Node {
        unsafe { (*self.node).mParent }.into()
    }

    pub fn get_transformation(&self) -> Matrix4x4 {
        unsafe { (*self.node).mTransformation }.into()
    }
}