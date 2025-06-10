use crate::node::Node;
use std::sync::atomic::{AtomicUsize, Ordering};
static LABEL_COUNTER: AtomicUsize = AtomicUsize::new(0);

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

// helper to emit code for if-else statements
fn emit_if(cond: &Node, then_stmt: &Node, else_stmt: Option<&Node>) {
    // Evaluate condition and pop into x0
    gen_node(cond);
    println!("    ldr x0, [sp], #16");
    // Compare with zero
    println!("    cmp x0, #0");
    // Generate unique labels
    let id = LABEL_COUNTER.fetch_add(1, Ordering::SeqCst);
    let else_label = format!(".Lelse{}", id);
    let end_label = format!(".Lend{}", id);
    // If zero, jump to else
    println!("    beq {}", else_label);
    // then branch
    gen_node(then_stmt);
    // Jump to end
    println!("    b {}", end_label);
    // else label
    println!("{}:", else_label);
    if let Some(es) = else_stmt {
        gen_node(es);
    }
    // end label
    println!("{}:", end_label);
}

fn emit_while(cond: &Node, body: &Node) {
    // while loop: .LloopX: if !(cond) break; body; b .LloopX; .LendX:
    let id = LABEL_COUNTER.fetch_add(1, Ordering::SeqCst);
    let loop_label = format!(".Lloop{}", id);
    let end_label = format!(".Lend{}", id);
    // loop start label
    println!("{}:", loop_label);
    // evaluate condition and pop into x0
    gen_node(cond);
    println!("    ldr x0, [sp], #16");
    println!("    cmp x0, #0");
    // if zero, jump to end
    println!("    beq {}", end_label);
    // loop body
    gen_node(body);
    // jump back to loop start
    println!("    b {}", loop_label);
    // end label
    println!("{}:", end_label);
}

// helper to emit code for for-loop statements
fn emit_for(init: &Node, cond: &Node, update: &Node, body: &Node) {
    // for(init; cond; update) body
    let id = LABEL_COUNTER.fetch_add(1, Ordering::SeqCst);
    let loop_label = format!(".Lfor{}", id);
    let cond_label = format!(".Lcond{}", id);
    let end_label = format!(".Lend{}", id);
    // init
    gen_node(init);
    println!("    ldr x0, [sp], #16");
    // jump to cond check
    println!("    b {}", cond_label);
    // loop body
    println!("{}:", loop_label);
    gen_node(body);
    // update
    gen_node(update);
    println!("    ldr x0, [sp], #16");
    // condition check
    println!("{}:", cond_label);
    gen_node(cond);
    println!("    ldr x0, [sp], #16");
    println!("    cmp x0, #0");
    println!("    bne {}", loop_label);
    // end label
    println!("{}:", end_label);
}

// helper to emit code for function call statements with arguments
fn emit_call(name: &String, args: &[Node]) {
    // evaluate arguments and push onto stack
    for arg in args {
        gen_node(arg);
    }
    // pop arguments into x registers (reverse order)
    for i in (0..args.len()).rev() {
        println!("    ldr x{}, [sp], #16", i);
    }
    // call external function (prepend underscore)
    println!("    bl _{}", name);
    // push return value onto stack
    println!("    str x0, [sp, #-16]!");
}

// helper to recursively generate code for each node
fn gen_node(node: &Node) {
    match node {
        Node::Seq(lhs, rhs) => emit_seq(lhs, rhs),
        Node::Num(n) => push_imm(*n),
        Node::Var(off) => emit_var(*off),
        Node::Call(name, args) => emit_call(name, args),
        Node::Return(node) => emit_return(node),
        Node::If(cond, then_stmt, else_stmt) => emit_if(cond, then_stmt, else_stmt.as_deref()),
        Node::While(cond, body) => emit_while(cond, body),
        Node::For(init, cond, update, body) => emit_for(init, cond, update, body),
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
