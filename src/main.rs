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
    let code: Vec<node::Node> = parser.program();

    // アセンブリの前半部分を出力
    println!(".intel_syntax noprefix");

    // 先頭の式から順にコードを生成
    for node in code {
        gen(node);
    }
}

fn gen(node: node::Node) {
    match node.kind {
        node::NodeKind::ND_NUM => {
            println!("  push {}", node.val.unwrap());
            return;
        },
        node::NodeKind::ND_LVAR => {
            gen_lval(node);
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
            return;
        },
        node::NodeKind::ND_ASSIGN => {
            gen_lval(*node.lhs.unwrap());
            gen(*node.rhs.unwrap());

            println!("  pop rdi");
            println!("  pop rax");
            println!("  mov [rax], rdi");
            println!("  push rdi");
            return;
        },
        node::NodeKind::ND_RETURN => {
            gen(*node.lhs.unwrap());

            println!("  pop rax");
            println!("  mov rsp, rbp");
            println!("  pop rbp");
            println!("  ret");
            return;
        },
        node::NodeKind::ND_IF => {
            let state_node: node::Node = *node.rhs.unwrap();
            let cnt: i32 = node.val.unwrap();
            gen(*node.lhs.unwrap());
            println!("  pop rax");
            println!("  cmp rax, 0");
            println!("  je .Lelse{}", cnt);
            gen(*state_node.lhs.unwrap());
            println!("  jmp .Lend{}", cnt);
            println!(".Lelse{}:", cnt);
            if state_node.rhs.is_some() {
                gen(*state_node.rhs.unwrap());
            }
            println!(".Lend{}:", cnt);
            return;
        },
        node::NodeKind::ND_WHILE => {
            let cnt: i32 = node.val.unwrap();
            println!(".Lbegin{}:", cnt);
            gen(*node.lhs.unwrap());
            println!("  pop rax");
            println!("  cmp rax, 0");
            println!("  je .Lend{}", cnt);
            gen(*node.rhs.unwrap());
            println!("  jmp .Lbegin{}", cnt);
            println!(".Lend{}:", cnt);
            return;
        },
        node::NodeKind::ND_FOR1 => {
            let cnt: i32 = node.val.unwrap();
            if node.lhs.is_some() {
                gen(*node.lhs.unwrap());
            }
            println!(".Lbegin{}:", cnt);
            let cond_node: node::Node = *node.rhs.unwrap();
            if cond_node.lhs.is_some() {
                gen(*cond_node.lhs.unwrap());
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je .Lend{}", cnt);
            } else {
                // 条件式がない場合は常にtrue
                println!("  jmp .Lend{}", cnt);
            }
            let inc_node: node::Node = *cond_node.rhs.unwrap();
            gen(*inc_node.rhs.unwrap());
            if inc_node.lhs.is_some() {
                gen(*inc_node.lhs.unwrap());
            }
            println!("  jmp .Lbegin{}", cnt);
            println!(".Lend{}:", cnt);
            return;
        },
        node::NodeKind::ND_BLOCK => {
            let mut node: node::Node = node;
            loop {
                match node.lhs {
                    Some(stmt_node) => {
                        gen(*stmt_node);
                        node = *node.rhs.unwrap();
                        println!("  pop rax");
                    },
                    None => { break; },
                }
            }
            return;
        },
        node::NodeKind::ND_CALL => {
            let regs = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
            let mut arg_node: node::Node = *node.lhs.unwrap();
            for reg in regs {
                if arg_node.lhs.is_none() { break; }
                gen(*arg_node.lhs.unwrap());
                println!("  pop {}", reg);
                arg_node = *arg_node.rhs.unwrap();
            }
            println!("  call {}", node.name.unwrap());
            println!("  push rax");
            return;
        },
        node::NodeKind::ND_FUNC => {
            let regs = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
            println!(".globl {}", node.name.clone().unwrap());
            println!("{}:", node.name.unwrap());
            println!("  push rbp");
            println!("  mov rbp, rsp");
            println!("  sub rsp, 208");

            // 引数をスタックにプッシュ
            let args_num: i32 = node.val.unwrap();
            for i in 0..args_num {
                println!("  mov [rbp-{}], {}", 8 * (i + 1), regs[i as usize]);
            }

            gen(*node.lhs.unwrap());


            println!("  mov rsp, rbp");
            println!("  pop rbp");
            println!("  ret");
            println!("");
            return;
        },
        node::NodeKind::ND_ADDR => {
            gen_lval(*node.lhs.unwrap());
            return;
        },
        node::NodeKind::ND_DEREF => {
            gen(*node.lhs.unwrap());
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
            return;
        },
        _ => {},
    }

    gen(*node.lhs.unwrap());
    gen(*node.rhs.unwrap());

    println!("  pop rdi");
    println!("  pop rax");

    match node.kind {
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
    return;
}

fn gen_lval(node: node::Node) -> node::Node {
    if !matches!(node.kind, node::NodeKind::ND_LVAR) {
        panic!("代入の左辺値が変数ではありません");
    }

    println!("  mov rax, rbp");
    println!("  sub rax, {}", node.offset.unwrap());
    println!("  push rax");

    node
}
