use rustc::node::*;
use rustc::token::*;
use std::env;

fn main() {
    let exp = env::args().nth(1).expect("Usage: program <exp>");

    let mut iter = tokenize(&exp).into_iter().peekable();

    let node = program(&mut iter);
    // let node = expr(&mut iter);
    // Generate the program
    generate(&node);
}
