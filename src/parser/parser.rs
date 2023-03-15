use crate::tokenizer::token;
use crate::node::node::{Node, NodeKind, new_node, new_node_num};

pub struct Parser {
    tokens: token::Tokens,
}

impl Parser {
    pub fn new(tokens: token::Tokens) -> Self {
        Parser{tokens: tokens}
    }

    pub fn expr(&mut self) -> Node {
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
        let mut node: Node = self.primary();

        loop {
            if self.tokens.consume(String::from("*")) {
                node = new_node(NodeKind::ND_MUL, node, self.primary());
            } else if self.tokens.consume(String::from("/")) {
                node = new_node(NodeKind::ND_DIV, node, self.primary());
            } else {
                return node;
            }
        }
    }

    pub fn primary(&mut self) -> Node {
        if self.tokens.consume(String::from("(")) {
            let node: Node = self.expr();
            self.tokens.expect(String::from(")"));
            return node;
        }

        new_node_num(self.tokens.expect_number())
    }
}

