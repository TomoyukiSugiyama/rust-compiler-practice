use crate::node::Node;

// helper to push an immediate onto the stack
fn push_imm(n: u64) {
    println!("    mov x0, #{}", n);
    println!("    str x0, [sp, #-16]!");
}

// helper to emit code for binary operations
fn emit_binop(op: &str, lhs: &Node, rhs: &Node) {
    gen_node(lhs);
    gen_node(rhs);
    println!("    ldr x1, [sp], #16");
    println!("    ldr x0, [sp], #16");
    println!("    {} x0, x0, x1", op);
    println!("    str x0, [sp, #-16]!");
}

// helper to recursively generate code for each node
fn gen_node(node: &Node) {
    match node {
        Node::Num(n) => push_imm(*n),
        Node::Add(lhs, rhs) => emit_binop("add", lhs, rhs),
        Node::Sub(lhs, rhs) => emit_binop("sub", lhs, rhs),
        Node::Mul(lhs, rhs) => emit_binop("mul", lhs, rhs),
        Node::Div(lhs, rhs) => emit_binop("sdiv", lhs, rhs),
        _ => unimplemented!("codegen for {:?}", node),
    }
}

/// Generate full ARM64 assembly for the AST, including prologue and epilogue.
pub fn generate(node: &Node) {
    // function prologue
    println!(".section __TEXT,__text");
    println!(".globl _main");
    println!("_main:");
    // generate body
    gen_node(node);
    // function epilogue: pop final result into x0 and return
    println!("    ldr x0, [sp], #16");
    println!("    ret");
}