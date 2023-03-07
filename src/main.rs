use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("引数の個数が正しくありません\n");
        return;
    }

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let v: Vec<char> = args[1].chars().collect();
    let mut num = 0;
    let mut prev = '#';
    for c in v {
        if c == '+' || c == '-' {
            if prev == '#' {
                println!("  mov rax, {}", num);
            } else if prev == '+' {
                println!("  add rax, {}", num);
            } else {
                println!("  sub rax, {}", num);
            }
            num = 0;
            prev = c;
        } else if '0' <= c && c <= '9' {
            num = num * 10 + (c as i32 - '0' as i32);
        } else {
            println!("予期しない文字です: {}", c);
            return;
        }
    }
    if prev == '#' {
        println!("  mov rax, {}", num);
    } else if prev == '+' {
        println!("  add rax, {}", num);
    } else {
        println!("  sub rax, {}", num);
    }
    println!("  ret");
}
