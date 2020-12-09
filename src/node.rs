use russimp_sys::{aiNode, aiMatrix4x4};

use crate::{
    FromRaw,
    scene::{PostProcessSteps, Scene},
};

use std::{
    rc::Rc,
    cell::RefCell
};
use crate::metadata::MetaData;

pub struct Node<'a> {
    node: &'a aiNode,
    pub name: String,
    pub children: Vec<Rc<RefCell<Node<'a>>>>,
    pub meshes: Vec<&'a u32>,
    pub metadata: Option<MetaData<'a>>,
    pub transformation: aiMatrix4x4,
}

impl<'a> FromRaw for Node<'a> {}

impl<'a> Into<Node<'a>> for &'a aiNode {
    fn into(self) -> Node<'a> {
        Node {
            node: self,
            name: self.mName.into(),
            children: Node::get_vec_rc_from_raw(self.mChildren, self.mNumChildren),
            meshes: Node::get_rawvec(self.mMeshes, self.mNumMeshes),
            metadata: Node::get_raw(self.mMetaData),
            transformation: self.mTransformation,
        }
    }
}

impl<'a> Node<'a> {
    fn get_parent(&self) -> Option<Node<'a>> {
        Node::get_raw(self.node.mParent)
    }
}

#[test]
fn checking_nodes() {
    let current_directory_buf = std::env::current_dir().unwrap().join("russimp-sys/assimp/test/models/BLEND/box.blend");

    let scene = Scene::from(current_directory_buf.to_str().unwrap(),
                            vec![PostProcessSteps::CalcTangentSpace,
                                 PostProcessSteps::Triangulate,
                                 PostProcessSteps::JoinIdenticalVertices,
                                 PostProcessSteps::SortByPType]).unwrap();

    let root = scene.root.as_ref().unwrap();
    let borrow = root.borrow();

    assert_eq!("<BlenderRoot>".to_string(), borrow.name);
    assert_eq!(3, borrow.children.len());

    let first_son = borrow.children[0].borrow();
    assert_eq!("Cube".to_string(), first_son.name);

    let second_son = borrow.children[1].borrow();
    assert_eq!("Lamp".to_string(), second_son.name);

    assert_eq!(0, borrow.meshes.len());

    assert!(borrow.metadata.is_none());

    assert_eq!(1.0, borrow.transformation.a1);
    assert_eq!(1.0, borrow.transformation.b2);
    assert_eq!(1.0, borrow.transformation.c3);
    assert_eq!(1.0, borrow.transformation.d4);
}

#[test]
fn childs_parent_name_matches() {
    let current_directory_buf = std::env::current_dir().unwrap().join("russimp-sys/assimp/test/models/BLEND/box.blend");

    let scene = Scene::from(current_directory_buf.to_str().unwrap(),
                            vec![PostProcessSteps::CalcTangentSpace,
                                 PostProcessSteps::Triangulate,
                                 PostProcessSteps::JoinIdenticalVertices,
                                 PostProcessSteps::SortByPType]).unwrap();

    let root = scene.root.as_ref().unwrap();
    let borrow = root.borrow();

    let first_son = borrow.children[0].borrow();
    let first_son_parent = first_son.get_parent().unwrap();

    assert_eq!(borrow.name, first_son_parent.name);
}