use crate::tokenizer::token;
use crate::node::node::{Node, NodeKind, new_node, new_node_alone, new_node_alone2, new_node_num, new_node_ident};

pub struct Parser {
    tokens: token::Tokens,
    locals: Vec<String>, // 変数文字列の一覧
}

impl Parser {
    pub fn new(tokens: token::Tokens) -> Self {
        Parser{tokens:tokens, locals:Vec::<String>::new()}
    }

    pub fn program(&mut self) -> Vec<Node> {
        let mut code: Vec<Node> = Vec::new();
        while !self.tokens.is_EOF() {
            code.push(self.stmt());
        }
        return code;
    }

    pub fn stmt(&mut self) -> Node {
        let node: Node;
        let kind: &token::TokenKind = &self.tokens.get_token().kind;
        match kind {
            token::TokenKind::TK_RETURN => {
                self.tokens.next();
                node = new_node_alone(NodeKind::ND_RETURN, self.expr());
                self.tokens.expect(String::from(";"));
            },
            token::TokenKind::TK_IF => {
                self.tokens.next();
                self.tokens.expect(String::from("("));
                let cond: Node = self.expr();
                self.tokens.expect(String::from(")"));
                node = new_node(NodeKind::ND_IF, cond, self.if_state());
            },
            token::TokenKind::TK_WHILE => {
                self.tokens.next();
                self.tokens.expect(String::from("("));
                let cond: Node = self.expr();
                self.tokens.expect(String::from(")"));
                node = new_node(NodeKind::ND_WHILE, cond, self.stmt());
            },
            token::TokenKind::TK_FOR => {
                self.tokens.next();
                self.tokens.expect(String::from("("));
                if self.tokens.consume(String::from(";")) {
                    node = new_node_alone2(NodeKind::ND_FOR1, self.for1());
                } else {
                    let init: Node = self.expr();
                    self.tokens.expect(String::from(";"));
                    node = new_node(NodeKind::ND_FOR1, init, self.for1());
                }
            },
            _ => {
                node = self.expr();
                self.tokens.expect(String::from(";"));
            },
        }
        return node;
    }

    pub fn if_state(&mut self) -> Node {
        let node: Node;
        let then: Node = self.stmt();
        let kind: &token::TokenKind = &self.tokens.get_token().kind;
        if matches!(kind, token::TokenKind::TK_ELSE) {
            self.tokens.next();
            node = new_node(NodeKind::ND_IFIN, then, self.stmt());
        } else {
            node = new_node_alone(NodeKind::ND_IFIN, then);
        }
        node
    }

    pub fn for1(&mut self) -> Node {
        let node: Node;
        if self.tokens.consume(String::from(";")) {
            node = new_node_alone2(NodeKind::ND_FOR2, self.for2());
        } else {
            let cond: Node = self.expr();
            self.tokens.expect(String::from(";"));
            node = new_node(NodeKind::ND_FOR2, cond, self.for2());
        }
        node
    }

    pub fn for2(&mut self) -> Node {
        let node: Node;
        if self.tokens.consume(String::from(")")) {
            node = new_node_alone2(NodeKind::ND_FOR3, self.stmt());
        } else {
            let inc: Node = self.expr();
            self.tokens.expect(String::from(")"));
            node = new_node(NodeKind::ND_FOR3, inc, self.stmt());
        }
        node
    }


    pub fn expr(&mut self) -> Node {
        return self.assign();
    }

    pub fn assign(&mut self) -> Node {
        let mut node: Node = self.equality();
        if self.tokens.consume(String::from("=")) {
            node = new_node(NodeKind::ND_ASSIGN, node, self.assign());
        }
        return node;
    }

    pub fn equality(&mut self) -> Node {
        let mut node: Node = self.relational();

        if self.tokens.consume(String::from("==")) {
            node = new_node(NodeKind::ND_NQ, node, self.relational());
        } else if self.tokens.consume(String::from("!=")) {
            node = new_node(NodeKind::ND_NE, node, self.relational());
        }

        return node;
    }

    pub fn relational(&mut self) -> Node {
        let mut node: Node = self.add();

        if self.tokens.consume(String::from("<")) {
            node = new_node(NodeKind::ND_LT, node, self.add());
        } else if self.tokens.consume(String::from(">")) {
            node = new_node(NodeKind::ND_LT, self.add(), node);
        } else if self.tokens.consume(String::from("<=")) {
            node = new_node(NodeKind::ND_LE, node, self.add());
        } else if self.tokens.consume(String::from(">=")) {
            node = new_node(NodeKind::ND_LE, self.add(), node);
        }

        return node;
    }

    pub fn add(&mut self) -> Node {
        let mut node: Node = self.mul();

        loop {
            if self.tokens.consume(String::from("+")) {
                node = new_node(NodeKind::ND_ADD, node, self.mul());
            } else if self.tokens.consume(String::from("-")) {
                node = new_node(NodeKind::ND_SUB, node, self.mul());
            } else {
                return node;
            }
        }
    }

    pub fn mul(&mut self) -> Node {
        let mut node: Node = self.unary();

        loop {
            if self.tokens.consume(String::from("*")) {
                node = new_node(NodeKind::ND_MUL, node, self.unary());
            } else if self.tokens.consume(String::from("/")) {
                node = new_node(NodeKind::ND_DIV, node, self.unary());
            } else {
                return node;
            }
        }
    }

    pub fn unary(&mut self) -> Node {
        if self.tokens.consume(String::from("+")) {
            return self.primary();
        } else if self.tokens.consume(String::from("-")) {
            let node = new_node(NodeKind::ND_SUB, new_node_num(0), self.primary());
            return node;
        } else {
            return self.primary();
        }
    }

    pub fn primary(&mut self) -> Node {
        if self.tokens.consume(String::from("(")) {
            let node: Node = self.expr();
            self.tokens.expect(String::from(")"));
            return node;
        }

        let next_token = self.tokens.consume_ident();
        match next_token {
            Some(var) => {
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
        for (i, var) in self.locals.iter().enumerate() {
            if matches!(&var, s) {
                return Some((i as i32+1)*8);
            }
        }
        return None;
    }
}

