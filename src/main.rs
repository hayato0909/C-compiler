use std::env;
use compiler::tokenizer::token;

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

    // アセンブリの前半部分を出力
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    // 最初のmov命令を出力
    println!("  mov rax, {}", tokens.expect_number());

    // '+ <数>' または '- <数>'というトークン列を消費
    while !tokens.is_EOF() {
        if tokens.consume(String::from("+")) {
            println!("  add rax, {}", tokens.expect_number());
        } else {
            tokens.expect(String::from("-"));
            println!("  sub rax, {}", tokens.expect_number());
        }
    }
    println!("  ret");
}
