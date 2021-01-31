use crate::{
    sys::{
        aiNode,
        aiMatrix4x4,
    },
    scene::{
        PostProcessSteps,
        Scene,
    },
    metadata::MetaData,
    Utils,
};

use std::{
    rc::Rc,
    cell::RefCell,
};

use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Node {
    pub name: String,
    pub children: Vec<Rc<RefCell<Node>>>,
    pub meshes: Vec<u32>,
    pub metadata: Option<MetaData>,
    pub transformation: aiMatrix4x4,
}

impl Node {
    pub fn new(node: &aiNode) -> Node {
        Node {
            name: node.mName.into(),
            children: Utils::get_vec_rc_from_raw(node.mChildren, node.mNumChildren, &Node::new),
            meshes: Utils::get_rawvec(node.mMeshes, node.mNumMeshes),
            metadata: Utils::get_raw(node.mMetaData, &MetaData::new),
            transformation: node.mTransformation,
        }
    }

    fn get_parent(&self) -> Option<Node> {
        Utils::get_raw(self.node.mParent)
    }
}

#[test]
fn checking_nodes() {
    let current_directory_buf = get_model("models/BLEND/box.blend");

    let scene = Scene::from(current_directory_buf.as_str(),
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
    let current_directory_buf = get_model("models/BLEND/box.blend");

    let scene = Scene::from(current_directory_buf.as_str(),
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