pub enum NodeKind {
    ND_ADD, // +
    ND_SUB, // -
    ND_MUL, // *
    ND_DIV, // /
    ND_ASSIGN, // =
    ND_LVAR, //ローカル変数
    ND_NUM, // integer
    ND_NQ, // ==
    ND_NE, // !=
    ND_LT, // <
    ND_LE, // <=
    ND_RETURN, // return文
}

pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub val: Option<i32>,
    pub offset: Option<i32>,
}

pub fn new_node(kind: NodeKind, lhs: Node, rhs: Node) -> Node {
    let node: Node = Node{kind:kind, lhs:Some(Box::new(lhs)), rhs:Some(Box::new(rhs)), val:None, offset:None};
    node
}

pub fn new_node_alone(kind: NodeKind, lhs: Node) -> Node {
    Node{kind:kind, lhs:Some(Box::new(lhs)), rhs:None, val:None, offset:None}
}

pub fn new_node_num(num: i32) -> Node {
    let node: Node = Node{kind:NodeKind::ND_NUM, lhs:None, rhs:None, val:Some(num), offset:None};
    node
}

pub fn new_node_ident(offset: i32) -> Node {
    let node: Node = Node{kind:NodeKind::ND_LVAR, lhs:None, rhs:None, val:None, offset:Some(offset)};
    node
}

