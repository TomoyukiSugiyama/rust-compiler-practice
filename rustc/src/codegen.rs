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

// helper to recursively generate code for each node
fn gen_node(node: &Node) {
    match node {
        Node::Num(n) => push_imm(*n),
        Node::Add(lhs, rhs) => emit_binop("add", lhs, rhs),
        Node::Sub(lhs, rhs) => emit_binop("sub", lhs, rhs),
        Node::Mul(lhs, rhs) => emit_binop("mul", lhs, rhs),
        Node::Div(lhs, rhs) => emit_binop("sdiv", lhs, rhs),
        Node::Assign(lhs, rhs) => {
            // evaluate RHS
            gen_node(rhs);
            // pop result into x1
            println!("    ldr x1, [sp], #16");
            // determine variable offset or error
            let off = match &**lhs {
                Node::Var(off) => *off,
                other => panic!("assignment to non-variable: {:?}", other),
            };
            // store into variable slot at negative offset from frame pointer
            println!("    str x1, [x29, #-{}]", off);
            // push assigned value back onto stack
            println!("    str x1, [sp, #-16]!");
        }
        Node::Var(offset) => {
            println!("    ldr x0, [x29, #-{}]", offset);
        }
        Node::Eq(lhs, rhs) => emit_cmp("eq", lhs, rhs),
        Node::Ne(lhs, rhs) => emit_cmp("ne", lhs, rhs),
        Node::Lt(lhs, rhs) => emit_cmp("lt", lhs, rhs),
        Node::Gt(lhs, rhs) => emit_cmp("gt", lhs, rhs),
        Node::Le(lhs, rhs) => emit_cmp("le", lhs, rhs),
        Node::Ge(lhs, rhs) => emit_cmp("ge", lhs, rhs),
    }
}

/// Generate full ARM64 assembly for the AST, including prologue and epilogue.
pub fn generate(node: &Node) {
    // function prologue
    println!(".section __TEXT,__text");
    println!(".globl _main");
    println!("_main:");
    // save old frame pointer and set up new
    println!("    stp x29, x30, [sp, #-16]!");
    println!("    mov x29, sp");
    // reserve space for 26 local variables (26*8 bytes)
    println!("    sub sp, sp, #208");
    // generate body
    gen_node(node);
    // function epilogue: pop final result into x0 and return
    println!("    ldr x0, [sp], #16");
    // deallocate local variable region
    println!("    add sp, sp, #208");
    // restore frame pointer and return
    println!("    ldp x29, x30, [sp], #16");
    println!("    ret");
}
