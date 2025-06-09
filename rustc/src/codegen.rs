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

// helper to emit code for comparisons, using cmp + cset
fn emit_cmp(cond: &str, lhs: &Node, rhs: &Node) {
    gen_node(lhs);
    gen_node(rhs);
    println!("    ldr x1, [sp], #16");
    println!("    ldr x0, [sp], #16");
    println!("    cmp x0, x1");
    println!("    cset x0, {}", cond);
    println!("    str x0, [sp, #-16]!");
}

// helper to emit code for assignments
fn emit_assign(lhs: &Node, rhs: &Node) {
    // evaluate RHS and pop into x1
    gen_node(rhs);
    println!("    ldr x1, [sp], #16");
    // determine variable offset or error
    let off = match lhs {
        Node::Var(off) => *off,
        other => panic!("assignment to non-variable: {:?}", other),
    };
    // store into variable slot at negative offset from frame pointer
    println!("    str x1, [x29, #-{}]", off);
    // push assigned value back onto stack
    println!("    str x1, [sp, #-16]!");
}

// helper to emit code for variable load
fn emit_var(off: u64) {
    // load variable from frame pointer
    println!("    ldr x0, [x29, #-{}]", off);
    // push loaded value onto stack
    println!("    str x0, [sp, #-16]!");
}

// helper to emit code for sequence of two nodes
fn emit_seq(lhs: &Node, rhs: &Node) {
    gen_node(lhs);
    // discard lhs result
    println!("    ldr x0, [sp], #16");
    gen_node(rhs);
}

// helper to emit code for return statement
fn emit_return(node: &Node) {
    gen_node(node);
    // pop return value into x0
    println!("    ldr x0, [sp], #16");
    // restore stack pointer to frame pointer
    println!("    mov sp, x29");
    // restore frame pointer and link register
    println!("    ldp x29, x30, [sp], #16");
    // return
    println!("    ret");
}

// helper to recursively generate code for each node
fn gen_node(node: &Node) {
    match node {
        Node::Seq(lhs, rhs) => emit_seq(lhs, rhs),
        Node::Num(n) => push_imm(*n),
        Node::Var(off) => emit_var(*off),
        Node::Return(node) => emit_return(node),
        Node::Assign(lhs, rhs) => emit_assign(lhs, rhs),
        Node::Add(lhs, rhs) => emit_binop("add", lhs, rhs),
        Node::Sub(lhs, rhs) => emit_binop("sub", lhs, rhs),
        Node::Mul(lhs, rhs) => emit_binop("mul", lhs, rhs),
        Node::Div(lhs, rhs) => emit_binop("sdiv", lhs, rhs),
        Node::Eq(lhs, rhs) => emit_cmp("eq", lhs, rhs),
        Node::Ne(lhs, rhs) => emit_cmp("ne", lhs, rhs),
        Node::Lt(lhs, rhs) => emit_cmp("lt", lhs, rhs),
        Node::Gt(lhs, rhs) => emit_cmp("gt", lhs, rhs),
        Node::Le(lhs, rhs) => emit_cmp("le", lhs, rhs),
        Node::Ge(lhs, rhs) => emit_cmp("ge", lhs, rhs),
    }
}

fn gen_prologue() {
    println!(".section __TEXT,__text");
    println!(".globl _main");
    println!("_main:");
    // save old frame pointer and set up new
    println!("    stp x29, x30, [sp, #-16]!");
    println!("    mov x29, sp");
    // reserve space for 26 local variables (26*8 bytes)
    println!("    sub sp, sp, #208");
}

fn gen_epilogue() {
    // pop return value into x0
    println!("    ldr x0, [sp], #16");
    // deallocate local variable region
    println!("    add sp, sp, #208");
    // restore frame pointer and return
    println!("    ldp x29, x30, [sp], #16");
    println!("    ret");
}

/// Generate full ARM64 assembly for the AST, including prologue and epilogue.
pub fn generate(node: &Node) {
    gen_prologue();
    gen_node(node);
    gen_epilogue();
}
