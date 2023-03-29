use crate::tokenizer::token;
use crate::node::node::{Node, NodeKind, new_node, new_node_alone, new_node_alone2, new_node_nothing, new_node_num, new_node_ident};

pub struct Parser {
    tokens: token::Tokens,
    locals: Vec<String>, // 変数文字列の一覧
    if_cnt: i32, // if文の数
    while_cnt: i32, // while文の数
    for_cnt: i32, // for文の数
}

impl Parser {
    pub fn new(tokens: token::Tokens) -> Self {
        Parser{tokens:tokens, locals:Vec::<String>::new(), if_cnt:0, while_cnt:0, for_cnt:0}
    }

    // program = stmt*
    pub fn program(&mut self) -> Vec<Node> {
        let mut code: Vec<Node> = Vec::new();
        while !self.tokens.is_eof() {
            code.push(self.stmt());
        }
        return code;
    }

    // stmt = expr ";"
    //     | "if" "(" expr ")" if_state
    //     | "while" "(" expr ")" stmt
    //     | "for" "(" expr? ";" for1
    //     | "{" stmt* "}"
    //     |  expr ";"
    pub fn stmt(&mut self) -> Node {
        let mut node: Node;
        let kind: &token::TokenKind = &self.tokens.get_token().kind;
        match kind {
            token::TokenKind::TK_RETURN => {
                self.tokens.next();
                node = new_node_alone(NodeKind::ND_RETURN, self.expr(), None::<i32>, None::<String>);
                self.tokens.expect(String::from(";"));
            },
            token::TokenKind::TK_IF => {
                self.tokens.next();
                self.tokens.expect(String::from("("));
                let cond: Node = self.expr();
                self.tokens.expect(String::from(")"));
                node = new_node(NodeKind::ND_IF, cond, self.if_state(), Some(self.if_cnt), None::<String>);
                self.if_cnt += 1;
            },
            token::TokenKind::TK_WHILE => {
                self.tokens.next();
                self.tokens.expect(String::from("("));
                let cond: Node = self.expr();
                self.tokens.expect(String::from(")"));
                node = new_node(NodeKind::ND_WHILE, cond, self.stmt(), Some(self.while_cnt), None::<String>);
                self.while_cnt += 1;
            },
            token::TokenKind::TK_FOR => {
                self.tokens.next();
                self.tokens.expect(String::from("("));
                if self.tokens.consume(String::from(";")) {
                    node = new_node_alone2(NodeKind::ND_FOR1, self.for1(), Some(self.for_cnt), None::<String>);
                } else {
                    let init: Node = self.expr();
                    self.tokens.expect(String::from(";"));
                    node = new_node(NodeKind::ND_FOR1, init, self.for1(), Some(self.for_cnt), None::<String>);
                }
                self.for_cnt += 1;
            },
            _ => {
                if self.tokens.consume(String::from("{")) {
                    let mut blocks: Vec<Node> = Vec::new();
                    while !self.tokens.consume(String::from("}")) {
                        blocks.push(self.stmt());
                    }

                    // 最後のノードを作成
                    node = new_node_nothing(NodeKind::ND_BLOCK, None::<i32>, None::<String>);           
                    // 逆順にノードを作成
                    blocks.reverse();
                    for block in blocks {
                        node = new_node(NodeKind::ND_BLOCK, block, node, None::<i32>, None::<String>);
                    }
                } else {
                    node = self.expr();
                    self.tokens.expect(String::from(";"));
                }
            },
        }
        return node;
    }

    // if_state = stmt ("else" stmt)?
    pub fn if_state(&mut self) -> Node {
        let node: Node;
        let then: Node = self.stmt();
        let kind: &token::TokenKind = &self.tokens.get_token().kind;
        if matches!(kind, token::TokenKind::TK_ELSE) {
            self.tokens.next();
            node = new_node(NodeKind::ND_IFIN, then, self.stmt(), None::<i32>, None::<String>);
        } else {
            node = new_node_alone(NodeKind::ND_IFIN, then, None::<i32>, None::<String>);
        }
        node
    }

    // for1 = expr? ";" for2
    pub fn for1(&mut self) -> Node {
        let node: Node;
        if self.tokens.consume(String::from(";")) {
            node = new_node_alone2(NodeKind::ND_FOR2, self.for2(), None::<i32>, None::<String>);
        } else {
            let cond: Node = self.expr();
            self.tokens.expect(String::from(";"));
            node = new_node(NodeKind::ND_FOR2, cond, self.for2(), None::<i32>, None::<String>);
        }
        node
    }

    // for = expr? ")" stmt
    pub fn for2(&mut self) -> Node {
        let node: Node;
        if self.tokens.consume(String::from(")")) {
            node = new_node_alone2(NodeKind::ND_FOR3, self.stmt(), None::<i32>, None::<String>);
        } else {
            let inc: Node = self.expr();
            self.tokens.expect(String::from(")"));
            node = new_node(NodeKind::ND_FOR3, inc, self.stmt(), None::<i32>, None::<String>);
        }
        node
    }


    // expr = assign
    pub fn expr(&mut self) -> Node {
        return self.assign();
    }

    // assign = equality ("=" assign)?
    pub fn assign(&mut self) -> Node {
        let mut node: Node = self.equality();
        if self.tokens.consume(String::from("=")) {
            node = new_node(NodeKind::ND_ASSIGN, node, self.assign(), None::<i32>, None::<String>);
        }
        return node;
    }

    // equality = relational ("==" relational | "!=" relational)?
    pub fn equality(&mut self) -> Node {
        let mut node: Node = self.relational();

        if self.tokens.consume(String::from("==")) {
            node = new_node(NodeKind::ND_NQ, node, self.relational(), None::<i32>, None::<String>);
        } else if self.tokens.consume(String::from("!=")) {
            node = new_node(NodeKind::ND_NE, node, self.relational(), None::<i32>, None::<String>);
        }

        return node;
    }

    // relational = add ("<" add | "<=" add | ">" add | ">=" add)?
    pub fn relational(&mut self) -> Node {
        let mut node: Node = self.add();

        if self.tokens.consume(String::from("<")) {
            node = new_node(NodeKind::ND_LT, node, self.add(), None::<i32>, None::<String>);
        } else if self.tokens.consume(String::from(">")) {
            node = new_node(NodeKind::ND_LT, self.add(), node, None::<i32>, None::<String>);
        } else if self.tokens.consume(String::from("<=")) {
            node = new_node(NodeKind::ND_LE, node, self.add(), None::<i32>, None::<String>);
        } else if self.tokens.consume(String::from(">=")) {
            node = new_node(NodeKind::ND_LE, self.add(), node, None::<i32>, None::<String>);
        }

        return node;
    }

    // add = mul ("+" mul | "-" mul)*
    pub fn add(&mut self) -> Node {
        let mut node: Node = self.mul();

        loop {
            if self.tokens.consume(String::from("+")) {
                node = new_node(NodeKind::ND_ADD, node, self.mul(), None::<i32>, None::<String>);
            } else if self.tokens.consume(String::from("-")) {
                node = new_node(NodeKind::ND_SUB, node, self.mul(), None::<i32>, None::<String>);
            } else {
                return node;
            }
        }
    }

    // mul = unary ("*" unary | "/" unary)*
    pub fn mul(&mut self) -> Node {
        let mut node: Node = self.unary();

        loop {
            if self.tokens.consume(String::from("*")) {
                node = new_node(NodeKind::ND_MUL, node, self.unary(), None::<i32>, None::<String>);
            } else if self.tokens.consume(String::from("/")) {
                node = new_node(NodeKind::ND_DIV, node, self.unary(), None::<i32>, None::<String>);
            } else {
                return node;
            }
        }
    }

    // unary = ("+" | "-")? primary
    pub fn unary(&mut self) -> Node {
        if self.tokens.consume(String::from("+")) {
            return self.primary();
        } else if self.tokens.consume(String::from("-")) {
            let node = new_node(NodeKind::ND_SUB, new_node_num(0), self.primary(), None::<i32>, None::<String>);
            return node;
        } else {
            return self.primary();
        }
    }

    // primary = num
    //      | ident ("(" ")")?
    //      | "(" expr ")"
    pub fn primary(&mut self) -> Node {
        if self.tokens.consume(String::from("(")) {
            let node: Node = self.expr();
            self.tokens.expect(String::from(")"));
            return node;
        }

        let next_token = self.tokens.consume_ident();
        match next_token {
            Some(var) => {
                if self.tokens.consume(String::from("(")) {
                    // ident "(" ")"
                    self.tokens.expect(String::from(")"));
                    return new_node_nothing(NodeKind::ND_CALL, None::<i32>, Some(var));
                } else {
                    // ident
                    let offset: i32;
                    match self.find_lvar(&var) {
                        Some(i) => {
                            offset = i;
                        },
                        None => {
                            self.locals.push(var);
                            offset = self.locals.len() as i32 * 8;
                        }
                    }
                    return new_node_ident(offset);
                }
            },
            None => {
                return new_node_num(self.tokens.expect_number());
            },
        }
    }

    // すでに出てきている変数の中に含まれているかを探す
    // 見つかった場合には、そのオフセットを返す
    // 見つからなかった場合には、Noneを返す
    fn find_lvar(&self, var: &String) -> Option<i32> {
        for (i, s) in self.locals.iter().enumerate() {
            if var == s {
                return Some((i as i32+1)*8);
            }
        }
        return None;
    }
}

