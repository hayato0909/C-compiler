use crate::tokenizer::token;
use crate::node::node::{Node, NodeKind, new_node, new_node_num, new_node_ident};

pub struct Parser {
    tokens: token::Tokens,
}

impl Parser {
    pub fn new(tokens: token::Tokens) -> Self {
        Parser{tokens: tokens}
    }

    pub fn program(&mut self) -> Vec<Node> {
        let mut code: Vec<Node> = Vec::new();
        while !self.tokens.is_EOF() {
            code.push(self.stmt());
        }
        return code;
    }

    pub fn stmt(&mut self) -> Node {
        let node: Node = self.expr();
        self.tokens.expect(String::from(";"));
        return node;
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
            Some(token) => {
                let offset: i32 = (token.s.chars().nth(0).unwrap() as i32 - 'a' as i32 + 1) * 8;
                return new_node_ident(offset);
            },
            None => {
                return new_node_num(self.tokens.expect_number());
            },
        }
    }
}

