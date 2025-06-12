use rustc::codegen::*;
use rustc::node::*;
use rustc::token::*;
use rustc::variable::Variable;
use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).expect("Usage: program <file>");
    let exp = fs::read_to_string(&filename).expect("Failed to read file");

    let mut iter = tokenize(&exp).into_iter().peekable();

    // variable context for parsing
    let mut vars = Variable::new("".to_string(), 0, None);
    // parse the program
    let node = program(&mut iter, &mut vars);
    // Generate the program
    generate(&node);
}
