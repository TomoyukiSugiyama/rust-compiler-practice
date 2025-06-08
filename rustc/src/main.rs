use rustc::node::*;
use rustc::*;
use std::env;

fn main() {
    let exp = env::args().nth(1).expect("Usage: program <exp>");

    let mut iter = tokenize(&exp).into_iter().peekable();





    let node = expr(&mut iter);
    println!(".section __TEXT,__text");
    println!(".globl _main");
    println!("_main:");

    generate(&node);
    // println!("node: {:?}", node);
    // First number
    // let first_tok = iter.next().unwrap();
    // let num = expect_number(&first_tok, &exp);
    // println!("    mov x0, #{}", num);
    // // Remaining operator-number pairs
    // while let Some(tok) = iter.next() {
    //     if at_eof(&tok) {
    //         break;
    //     }
    //     let op = expect_operator(&tok, &exp);
    //     let num_tok = iter.next().unwrap();
    //     let num = expect_number(&num_tok, &exp);
    //     if op == '+' {
    //         println!("    add x0, x0, #{}", num);
    //     } else {
    //         println!("    sub x0, x0, #{}", num);
    //     }
    // }
    // Pop final result and restore stack pointer
    println!("    ldr x0, [sp]");
    println!("    add sp, sp, #16");
    println!("    ret");


}
