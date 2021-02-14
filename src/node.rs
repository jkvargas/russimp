use crate::{
    metadata::MetaData,
    scene::{PostProcessSteps, Scene},
    sys::{aiMatrix4x4, aiNode},
    Matrix4x4, Utils,
};

use std::{cell::RefCell, ptr::slice_from_raw_parts, rc::Rc};

use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Node {
    pub name: String,
    pub children: Vec<Rc<RefCell<Node>>>,
    pub meshes: Vec<u32>,
    pub metadata: Option<MetaData>,
    pub transformation: Matrix4x4,
    #[derivative(Debug = "ignore")]
    pub parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(node: &aiNode) -> Rc<RefCell<Node>> {
        Self::go_through(node, None)
    }

    fn go_through(node: &aiNode, parent: Option<Rc<RefCell<Node>>>) -> Rc<RefCell<Node>> {
        // current simple node
        let res_node = Rc::new(RefCell::new(Self::create_simple_node(node)));

        let slice = slice_from_raw_parts(node.mChildren, node.mNumChildren as usize);
        if !slice.is_null() {
            let raw = unsafe { slice.as_ref() }.unwrap();

            for children in raw {
                let children_ref = unsafe { children.as_ref() }.unwrap();
                let res_children_node = Self::go_through(children_ref, Some(res_node.clone()));

                let mut result_borrow = res_node.borrow_mut();
                result_borrow.children.push(res_children_node);
            }
        }

        {
            let mut borrow_mut = res_node.borrow_mut();
            borrow_mut.parent = parent;
        }

        res_node
    }

    fn create_simple_node(node: &aiNode) -> Node {
        Node {
            name: node.mName.into(),
            children: Vec::new(),
            meshes: Utils::get_rawvec(node.mMeshes, node.mNumMeshes),
            metadata: Utils::get_raw(node.mMetaData, &MetaData::new),
            transformation: Matrix4x4::new(&node.mTransformation),
            parent: None,
        }
    }
}

#[test]
fn checking_nodes() {
    let current_directory_buf = Utils::get_model("models/BLEND/box.blend");

    let scene = Scene::from(
        current_directory_buf.as_str(),
        vec![
            PostProcessSteps::CalculateTangentSpace,
            PostProcessSteps::Triangulate,
            PostProcessSteps::JoinIdenticalVertices,
            PostProcessSteps::SortByPrimitiveType,
        ],
    )
    .unwrap();

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
    let current_directory_buf = Utils::get_model("models/BLEND/box.blend");

    let scene = Scene::from(
        current_directory_buf.as_str(),
        vec![
            PostProcessSteps::CalculateTangentSpace,
            PostProcessSteps::Triangulate,
            PostProcessSteps::JoinIdenticalVertices,
            PostProcessSteps::SortByPrimitiveType,
        ],
    )
    .unwrap();

    let root = scene.root.as_ref().unwrap();
    let borrow = root.borrow();

    let first_son = borrow.children[0].borrow();
    let first_son_parent = first_son.parent.as_ref().unwrap();

    let dad = first_son_parent.borrow();

    assert_eq!(borrow.name, dad.name);
}

#[test]
fn debug_root() {
    let current_directory_buf = Utils::get_model("models/BLEND/box.blend");

    let scene = Scene::from(
        current_directory_buf.as_str(),
        vec![
            PostProcessSteps::CalculateTangentSpace,
            PostProcessSteps::Triangulate,
            PostProcessSteps::JoinIdenticalVertices,
            PostProcessSteps::SortByPrimitiveType,
        ],
    )
    .unwrap();

    dbg!(&scene.root);
}
