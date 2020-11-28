use russimp_sys::aiNode;

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