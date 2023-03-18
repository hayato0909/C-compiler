use std::env;
use compiler::tokenizer::token;
use compiler::node::node;
use compiler::parser::parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("引数の個数が正しくありません\n");
        return;
    }

    let v: Vec<char> = args[1].chars().collect();
    let mut tokens: token::Tokens = token::Tokens::new();
    // トークナイズする
    tokens.tokenize(v);
    let mut parser: parser::Parser = parser::Parser::new(tokens);
    let node: node::Node = parser.expr();

    // アセンブリの前半部分を出力
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    gen(node);

    println!("  pop rax");
    println!("  ret");
}

fn gen(node: node::Node) {
    let kind: node::NodeKind = node.kind;
    if matches!(kind, node::NodeKind::ND_NUM) {
        println!("  push {}", node.val.unwrap());
        return;
    }

    gen(*node.lhs.unwrap());
    gen(*node.rhs.unwrap());

    println!("  pop rdi");
    println!("  pop rax");

    match kind {
        node::NodeKind::ND_ADD => { println!("  add rax, rdi"); },
        node::NodeKind::ND_SUB => { println!("  sub rax, rdi"); },
        node::NodeKind::ND_MUL => { println!("  imul rax, rdi"); },
        node::NodeKind::ND_DIV => {
            println!("  cqo");
            println!("  idiv rdi");
        },
        node::NodeKind::ND_NQ => {
            println!("  cmp rax, rdi");
            println!("  sete al");
            println!("  movzb rax, al");
        },
        node::NodeKind::ND_NE => {
            println!("  cmp rax, rdi");
            println!("  setne al");
            println!("  movzb rax, al");
        },
        node::NodeKind::ND_LT => {
            println!("  cmp rax, rdi");
            println!("  setl al");
            println!("  movzb rax, al");
        },
        node::NodeKind::ND_LE => {
            println!("  cmp rax, rdi");
            println!("  setle al");
            println!("  movzb rax, al");
        },
        _ => {},
    }

    println!("  push rax");
}
