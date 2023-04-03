#[allow(non_camel_case_types)]
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
    ND_IF, // if文 (lhs: if条件, rhs: if実行文ノード)
    ND_IFIN, // if 実行文内部 (lhs: if実行文ノード, rhs: else実行文ノード)
    ND_WHILE, // while文 (lhs: while条件, rhs: while実行文ノード)
    ND_FOR1, // for文 (lhs: 初期化式, rhs: for2 node)
    ND_FOR2, // for文 (lhs: 条件式, rhs: for3 node) 
    ND_FOR3, // for文 (lhs: 更新式, rhs: for実行文ノード)
    ND_BLOCK, // ブロック (lhs: ブロック内部のノード, rhs: 次のノード)
    ND_CALL, // 関数呼び出し (lhs: 引数の連結ノード)
    ND_FUNC, // 関数定義 (lhs: block, val: 引数の個数, name: 関数名)
    ND_ADDR, // アドレス演算子 (&)
    ND_DEREF, // ポインタ演算子 (*)
    ND_VARDEF, // 変数定義 (name: 変数名)
}

pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub val: Option<i32>,
    pub offset: Option<i32>,
    pub name: Option<String>,
}

pub fn new_node(kind: NodeKind, lhs: Node, rhs: Node, cnt: Option<i32>, s: Option<String>) -> Node {
    let node: Node = Node{kind:kind, lhs:Some(Box::new(lhs)), rhs:Some(Box::new(rhs)), val:cnt, offset:None, name:s};
    node
}

pub fn new_node_alone(kind: NodeKind, lhs: Node, cnt: Option<i32>, s:Option<String>) -> Node {
    Node{kind:kind, lhs:Some(Box::new(lhs)), rhs:None, val:cnt, offset:None, name:s}
}

pub fn new_node_alone2(kind: NodeKind, rhs: Node, cnt: Option<i32>, s:Option<String>) -> Node {
    Node{kind:kind, lhs:None, rhs:Some(Box::new(rhs)), val:cnt, offset:None, name:s}
}

pub fn new_node_nothing(kind: NodeKind, cnt: Option<i32>, s: Option<String>) -> Node {
    Node{kind:kind, lhs:None, rhs:None, val:cnt, offset:None, name:s}
}

pub fn new_node_num(num: i32) -> Node {
    let node: Node = Node{kind:NodeKind::ND_NUM, lhs:None, rhs:None, val:Some(num), offset:None, name:None};
    node
}

pub fn new_node_ident(offset: i32) -> Node {
    let node: Node = Node{kind:NodeKind::ND_LVAR, lhs:None, rhs:None, val:None, offset:Some(offset), name:None};
    node
}

