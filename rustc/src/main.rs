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
    // Pop final result and restore stack pointer
    println!("    ldr x0, [sp]");
    println!("    add sp, sp, #16");
    println!("    ret");


}
