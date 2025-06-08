use crate::{TokenIter, TokenKind};
use std::iter::Peekable;

#[derive(Debug)]
pub enum Node {
    Num(u64),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
}

pub fn expr(toks: &mut Peekable<TokenIter>) -> Node {
    let mut lhs = mul(toks);
    while let Some(tok) = toks.peek() {
        match tok.kind {
            TokenKind::Operator('+') => {
                toks.next();
                lhs = Node::Add(Box::new(lhs), Box::new(mul(toks)));
            }
            TokenKind::Operator('-') => {
                toks.next();
                lhs = Node::Sub(Box::new(lhs), Box::new(mul(toks)));
            }
            _ => break,
        }
    }
    lhs
}

fn mul(toks: &mut Peekable<TokenIter>) -> Node {
    let mut lhs = primary(toks);
    while let Some(tok) = toks.peek() {
        match tok.kind {
            TokenKind::Operator('*') => {
                toks.next();
                lhs = Node::Mul(Box::new(lhs), Box::new(primary(toks)));
            }
            TokenKind::Operator('/') => {
                toks.next();
                lhs = Node::Div(Box::new(lhs), Box::new(primary(toks)));
            }
            _ => break,
        }
    }
    lhs
}

fn primary(toks: &mut Peekable<TokenIter>) -> Node {
    let tok = toks.next().unwrap();
    if let TokenKind::Number(n) = tok.kind {
        Node::Num(n)
    } else {
        unreachable!()
    }
}

pub fn generate(node: &Node) {
    match node {
        Node::Num(n) => {
            println!("    mov x0, #{}", n);
            println!("    sub sp, sp, #16");
            println!("    str x0, [sp]");
        }
        Node::Add(lhs, rhs) => {
            generate(lhs);
            generate(rhs);
            println!("    ldr x1, [sp]");
            println!("    add sp, sp, #16");
            println!("    ldr x0, [sp]");
            println!("    add sp, sp, #16");
            println!("    add x0, x0, x1");
            println!("    sub sp, sp, #16");
            println!("    str x0, [sp]");
        }
        Node::Sub(lhs, rhs) => {
            generate(lhs);
            generate(rhs);
            println!("    ldr x1, [sp]");
            println!("    add sp, sp, #16");
            println!("    ldr x0, [sp]");
            println!("    add sp, sp, #16");
            println!("    sub x0, x0, x1");
            println!("    sub sp, sp, #16");
            println!("    str x0, [sp]");
        }
        Node::Mul(lhs, rhs) => {
            generate(lhs);
            generate(rhs);
            println!("    ldr x1, [sp]");
            println!("    add sp, sp, #16");
            println!("    ldr x0, [sp]");
            println!("    add sp, sp, #16");
            println!("    mul x0, x0, x1");
            println!("    sub sp, sp, #16");
            println!("    str x0, [sp]");
        }
        Node::Div(lhs, rhs) => {
            generate(lhs);
            generate(rhs);
            println!("    ldr x1, [sp]");
            println!("    add sp, sp, #16");
            println!("    ldr x0, [sp]");
            println!("    add sp, sp, #16");
            println!("    sdiv x0, x0, x1");
            println!("    sub sp, sp, #16");
            println!("    str x0, [sp]");
        }
    }
}
