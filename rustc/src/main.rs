use rustc::*;
use std::env;

fn main() {
    let exp = env::args().nth(1).expect("Usage: program <exp>");

    // Use tokenizer from library to split into numbers and operators
    let mut token = tokenize(&exp);

    println!(".section __TEXT,__text");
    println!(".globl _main");
    println!("_main:");
    token = consume(token).unwrap();
    let num = expect_number(&token, &exp);
    println!("    mov x0, #{}", num);
    while !at_eof(&token) {
        token = consume(token).unwrap();
        if at_eof(&token) {
            break;
        }
        let op = expect_operator(&token, &exp);
        token = consume(token).unwrap();
        if op == '+' {
            let num = expect_number(&token, &exp);
            println!("    add x0, x0, #{}", num);
        } else {
            let num = expect_number(&token, &exp);
            println!("    sub x0, x0, #{}", num);
        }
    }
    println!("    ret");
}
