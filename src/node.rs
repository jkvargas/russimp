use crate::{metadata::MetaData, sys::aiNode, *};
use derivative::Derivative;
use std::{cell::RefCell, rc::{Rc, Weak}, borrow::{BorrowMut, Borrow}, ops::{DerefMut, Deref}};

#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct Node {
    pub name: String,
    pub children: RefCell<Vec<Rc<Node>>>,
    pub meshes: Vec<u32>,
    pub metadata: Option<MetaData>,
    pub transformation: Matrix4x4,
    #[derivative(Debug = "ignore")]
    pub parent: RefCell<Weak<Node>>,
}

impl Node {
    pub(crate) fn new(node: &aiNode) -> Rc<Node> {
        Self::allocate(node, None)
    }

    fn allocate(node: &aiNode, parent: Option<&Rc<Node>>) -> Rc<Node> {
        // current simple node
        let mut res_node = Rc::new(Self::create_simple_node(node, parent));
        let nodes = utils::get_base_type_vec_from_raw(node.mChildren, node.mNumChildren);

        for children_ref in nodes {
            let child_node = Self::allocate(children_ref, Some(&res_node));
            res_node.borrow_mut().children.borrow_mut().deref_mut().push(child_node);
        }

        res_node
    }

    fn create_simple_node(node: &aiNode, parent: Option<&Rc<Node>>) -> Node {
        Node {
            name: node.mName.into(),
            children: RefCell::new(Vec::new()),
            meshes: utils::get_raw_vec(node.mMeshes, node.mNumMeshes),
            metadata: utils::get_raw(node.mMetaData),
            transformation: (&node.mTransformation).into(),
            parent: RefCell::new(parent.map(Rc::downgrade).unwrap_or_else(|| Weak::new()))
        }
    }
}

#[test]
fn checking_nodes() {
    use crate::scene::{PostProcess, Scene};

    let current_directory_buf = utils::get_model("models/BLEND/box.blend");

    let scene = Scene::from_file(
        current_directory_buf.as_str(),
        &[
            PostProcess::CalculateTangentSpace,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::SortByPrimitiveType,
        ],
    )
    .unwrap();

    let root = scene.root.as_ref().unwrap();
    let borrow = root;

    let children = borrow.children.borrow();
    assert_eq!("<BlenderRoot>".to_string(), borrow.name);
    assert_eq!(3, children.len());

    let first_son = &children[0];
    assert_eq!("Cube".to_string(), first_son.name);

    let second_son = &children[1];
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
    use crate::scene::{PostProcess, Scene};

    let current_directory_buf = utils::get_model("models/BLEND/box.blend");

    let scene = Scene::from_file(
        current_directory_buf.as_str(),
        &[
            PostProcess::CalculateTangentSpace,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::SortByPrimitiveType,
        ],
    )
    .unwrap();

    let root = scene.root.as_ref().unwrap().as_ref();
    assert!(root.parent.borrow().upgrade().is_none());

    let borrow = root.children.borrow();
    let children = borrow.deref();
    let first_son = &children[0];
    let first_son_parent = first_son.parent.borrow().upgrade().unwrap();

    assert_eq!(root.name, first_son_parent.name);
}

#[test]
fn debug_root() {
    use crate::scene::{PostProcess, Scene};

    let current_directory_buf = utils::get_model("models/BLEND/box.blend");

    let scene = Scene::from_file(
        current_directory_buf.as_str(),
        &[
            PostProcess::CalculateTangentSpace,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::SortByPrimitiveType,
        ],
    )
    .unwrap();

    dbg!(&scene.root);
}