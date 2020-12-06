use russimp_sys::aiNode;

use crate::{
    FromRaw,
    scene::{PostProcessSteps, Scene},
};

use std::rc::Rc;
use std::cell::RefCell;

pub struct Node<'a> {
    node: &'a aiNode,
    pub name: String,
    pub children: Vec<Rc<RefCell<Node<'a>>>>,
    pub meshes: Vec<&'a u32>,
    pub parent: Option<Rc<RefCell<Node<'a>>>>
}

impl<'a> FromRaw for Node<'a> {}

impl<'a> Into<Node<'a>> for &'a aiNode {
    fn into(self) -> Node<'a> {
        Node {
            node: self,
            name: self.mName.into(),
            children: Node::get_vec_rc_from_raw(self.mChildren, self.mNumChildren),
            meshes: Node::get_rawvec(self.mMeshes, self.mNumMeshes),
            parent: Node::get_rc_raw(self.mParent),
        }
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

    dbg!(&first_son.name);
}