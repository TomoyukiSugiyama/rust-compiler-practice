use crate::{TokenIter, TokenKind};
use std::iter::Peekable;

#[derive(Debug)]
pub enum NodeKind {
    Num(u64),
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
}

fn new_node(kind: NodeKind, lhs: Node, rhs: Node) -> Node {
    Node {
        kind,
        lhs: Some(Box::new(lhs)),
        rhs: Some(Box::new(rhs)),
    }
}

pub fn expr(toks: &mut Peekable<TokenIter>) -> Node {
    let mut lhs = mul(toks);
    while let Some(tok) = toks.peek() {
        if tok.kind == TokenKind::Operator('+') {
            toks.next();
            lhs = new_node(NodeKind::Add, lhs, mul(toks));
        } else if tok.kind == TokenKind::Operator('-') {
            toks.next();
            lhs = new_node(NodeKind::Sub, lhs, mul(toks));
        } else {
            break;
        }
    }
    lhs
}

fn mul(toks: &mut Peekable<TokenIter>) -> Node {
    let mut lhs = primary(toks);
    while let Some(tok) = toks.peek() {
        if tok.kind == TokenKind::Operator('*') {
            toks.next();
            lhs = new_node(NodeKind::Mul, lhs, primary(toks));
        } else if tok.kind == TokenKind::Operator('/') {
            toks.next();
            lhs = new_node(NodeKind::Div, lhs, primary(toks));
        } else {
            break;
        }
    }
    lhs
}

fn primary(toks: &mut Peekable<TokenIter>) -> Node {
    let tok = toks.next().unwrap();
    match tok.kind {
        TokenKind::Number(n) => Node {
            kind: NodeKind::Num(n),
            lhs: None,
            rhs: None,
        },
        _ => unreachable!(),
    }
}

pub fn generate(node: &Node) {
    match &node.kind {
        NodeKind::Num(n) => {
            // Load immediate and push to stack
            println!("    mov x0, #{}", n);
            println!("    sub sp, sp, #16");
            println!("    str x0, [sp]");
        }
        NodeKind::Add => {
            generate(node.lhs.as_ref().unwrap());
            generate(node.rhs.as_ref().unwrap());
            // Pop right into x1
            println!("    ldr x1, [sp]");
            println!("    add sp, sp, #16");
            // Pop left into x0
            println!("    ldr x0, [sp]");
            println!("    add sp, sp, #16");
            println!("    add x0, x0, x1");
            // Push result
            println!("    sub sp, sp, #16");
            println!("    str x0, [sp]");
        }
        NodeKind::Sub => {
            generate(node.lhs.as_ref().unwrap());
            generate(node.rhs.as_ref().unwrap());
            println!("    ldr x1, [sp]");
            println!("    add sp, sp, #16");
            println!("    ldr x0, [sp]");
            println!("    add sp, sp, #16");
            println!("    sub x0, x0, x1");
            println!("    sub sp, sp, #16");
            println!("    str x0, [sp]");
        }
        NodeKind::Mul => {
            generate(node.lhs.as_ref().unwrap());
            generate(node.rhs.as_ref().unwrap());
            println!("    ldr x1, [sp]");
            println!("    add sp, sp, #16");
            println!("    ldr x0, [sp]");
            println!("    add sp, sp, #16");
            println!("    mul x0, x0, x1");
            println!("    sub sp, sp, #16");
            println!("    str x0, [sp]");
        }
        NodeKind::Div => {
            generate(node.lhs.as_ref().unwrap());
            generate(node.rhs.as_ref().unwrap());
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
