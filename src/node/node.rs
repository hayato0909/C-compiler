pub enum NodeKind {
    ND_ADD,
    ND_SUB,
    ND_MUL,
    ND_DIV,
    ND_NUM,
}

pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub val: Option<i32>,
}

pub fn new_node(kind: NodeKind, lhs: Node, rhs: Node) -> Node {
    let node: Node = Node{kind:kind, lhs:Some(Box::new(lhs)), rhs:Some(Box::new(rhs)), val:None};
    node
}

pub fn new_node_num(num: i32) -> Node {
    let node: Node = Node{kind:NodeKind::ND_NUM, lhs:None, rhs:None, val:Some(num)};
    node
}

