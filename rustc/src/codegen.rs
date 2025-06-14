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
        Node::Var { offset } => *offset,
        other => panic!("assignment to non-variable: {:?}", other),
    };
    // store into variable slot via register-based addressing (handles large offsets)
    println!("    mov x2, x29");
    println!("    sub x2, x2, #{}", off);
    println!("    str x1, [x2]");
    // push assigned value back onto stack
    println!("    str x1, [sp, #-16]!");
}

// helper to emit code for variable load
fn emit_var(off: u64) {
    // load variable via register-based addressing (handles large offsets)
    println!("    mov x2, x29");
    println!("    sub x2, x2, #{}", off);
    println!("    ldr x0, [x2]");
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

// helper to emit code for system calls
fn emit_syscall(name: &String, args: &[Node]) {
    // evaluate arguments and push onto stack
    for arg in args {
        gen_node(arg);
    }
    // pop arguments into x registers (reverse order)
    for i in (0..args.len()).rev() {
        println!("    ldr x{}, [sp], #16", i);
    }
    // Set up system call number in x16
    match name.as_str() {
        "write" => {
            // For write syscall:
            // x0 = file descriptor (1 for stdout)
            // x1 = buffer address
            // x2 = buffer length
            println!("    mov x1, x0"); // Move string address to x1
            println!("    mov x0, #1"); // stdout file descriptor
            // Calculate string length
            println!("    mov x2, #0"); // Initialize length counter
            println!("    mov x3, x1"); // Copy string address to x3
            let id = LABEL_COUNTER.fetch_add(1, Ordering::SeqCst);
            let loop_label = format!(".Lstrlen_loop{}", id);
            let end_label = format!(".Lstrlen_end{}", id);
            println!("{}:", loop_label); // Label for string length calculation loop
            println!("    ldrb w4, [x3], #1"); // Load byte and increment pointer
            println!("    cbz w4, {}", end_label); // If zero (null terminator), exit loop
            println!("    add x2, x2, #1"); // Increment length counter
            println!("    b {}", loop_label); // Branch back to loop start
            println!("{}:", end_label); // Label for loop end
            println!("    movz x16, #0x0004, lsl #0"); // Set lower 16 bits
            println!("    movk x16, #0x2000, lsl #16"); // Set upper 16 bits
        }
        _ => panic!("unsupported system call: {}", name),
    }
    // Make the system call
    println!("    svc #0x80");
    // Push return value onto stack
    println!("    str x0, [sp, #-16]!");
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
    // Save caller-saved registers
    println!("    stp x29, x30, [sp, #-16]!");
    println!("    bl _{}", name);
    // Restore caller-saved registers
    println!("    ldp x29, x30, [sp], #16");
    // Push return value onto stack
    println!("    str x0, [sp, #-16]!");
}

// Compute maximum stack offset needed for local variables and arrays
fn compute_max_offset(node: &Node) -> u64 {
    match node {
        Node::Seq { first, second } => {
            let m1 = compute_max_offset(first);
            let m2 = compute_max_offset(second);
            if m1 > m2 { m1 } else { m2 }
        }
        Node::Function { body, .. } => compute_max_offset(body),
        Node::Num { .. } | Node::StringSlice { .. } => 0,
        Node::Var { offset } => *offset,
        Node::Call { args, .. } | Node::Syscall { args, .. } => {
            let mut m = 0;
            for arg in args {
                let mm = compute_max_offset(arg);
                if mm > m {
                    m = mm;
                }
            }
            m
        }
        Node::Assign { lhs, rhs } => compute_max_offset(lhs).max(compute_max_offset(rhs)),
        Node::Add { lhs, rhs }
        | Node::Sub { lhs, rhs }
        | Node::Mul { lhs, rhs }
        | Node::Div { lhs, rhs }
        | Node::Eq { lhs, rhs }
        | Node::Ne { lhs, rhs }
        | Node::Lt { lhs, rhs }
        | Node::Gt { lhs, rhs }
        | Node::Le { lhs, rhs }
        | Node::Ge { lhs, rhs } => compute_max_offset(lhs).max(compute_max_offset(rhs)),
        Node::Return { expr } | Node::Deref { expr } | Node::Addr { expr } => {
            compute_max_offset(expr)
        }
        Node::If {
            cond,
            then_stmt,
            else_stmt,
        } => {
            let mut m = compute_max_offset(cond).max(compute_max_offset(then_stmt));
            if let Some(e) = else_stmt {
                m = m.max(compute_max_offset(e));
            }
            m
        }
        Node::While { cond, body } => compute_max_offset(cond).max(compute_max_offset(body)),
        Node::For {
            init,
            cond,
            update,
            body,
        } => {
            let mut m = compute_max_offset(init);
            m = m.max(compute_max_offset(cond));
            m = m.max(compute_max_offset(update));
            m = m.max(compute_max_offset(body));
            m
        }
        Node::ArrayAssign { offset, elements } => {
            let mut m = *offset;
            if !elements.is_empty() {
                let eo = *offset + (elements.len() as u64 - 1) * 8;
                if eo > m {
                    m = eo;
                }
            }
            for elem in elements {
                let mm = compute_max_offset(elem);
                if mm > m {
                    m = mm;
                }
            }
            m
        }
    }
}

fn gen_prologue(name: &String, frame_size: u64) {
    println!(".globl _{}", name);
    println!("_{}:", name);
    // save old frame pointer and set up new
    println!("    stp x29, x30, [sp, #-16]!");
    println!("    mov x29, sp");
    // reserve space for local variables
    println!("    sub sp, sp, #{}", frame_size);
    println!("    str x0, [x29, #-8]");
}

fn gen_epilogue(frame_size: u64) {
    // pop return value into x0
    println!("    ldr x0, [sp], #16");
    // deallocate local variable region
    println!("    add sp, sp, #{}", frame_size);
    // restore frame pointer and return
    println!("    ldp x29, x30, [sp], #16");
    println!("    ret");
}

// helper to emit code for function definitions
fn emit_function(name: &String, args: &Vec<Node>, body: &Box<Node>) {
    // compute required frame size based on arguments and body
    let mut max_offset = 0u64;
    for arg in args.iter() {
        if let Node::Var { offset } = arg {
            if *offset > max_offset {
                max_offset = *offset;
            }
        }
    }
    let body_max = compute_max_offset(body);
    if body_max > max_offset {
        max_offset = body_max;
    }
    // align frame size to 16 bytes, at least 48
    let frame_size = if max_offset > 48 {
        ((max_offset + 15) / 16) * 16
    } else {
        48
    };
    gen_prologue(name, frame_size);
    // Save arguments to local variables
    for (i, arg) in args.iter().enumerate() {
        if let Node::Var { offset } = arg {
            println!("    str x{}, [x29, #-{}]", i, offset);
        }
    }
    gen_node(body);
    gen_epilogue(frame_size);
}

// helper to emit code for dereference
fn emit_deref(node: &Node) {
    gen_node(node);
    println!("    ldr x0, [sp], #16");
    println!("    ldr x0, [x0]");
    println!("    str x0, [sp, #-16]!");
}

// helper to emit code for address-of
fn emit_addr(node: &Node) {
    match node {
        Node::Var { offset } => {
            println!("    mov x0, x29");
            println!("    sub x0, x0, #{}", offset);
            println!("    str x0, [sp, #-16]!");
        }
        Node::Deref { expr } => {
            gen_node(expr);
        }
        _ => panic!("address-of not supported for {:?}", node),
    }
}

// helper to emit code for string literals
fn emit_string(s: &str) {
    // Generate a unique label for this string
    let id = LABEL_COUNTER.fetch_add(1, Ordering::SeqCst);
    let label = format!(".L.str.{}", id);

    // Emit the string data in the data section
    println!(".section __DATA,__data");
    println!("{}:", label);
    println!("    .asciz \"{}\"", s);

    // Switch back to text section
    println!(".section __TEXT,__text");

    // Load the address of the string into x0 (this represents the &str)
    println!("    adrp x0, {}@PAGE", label);
    println!("    add x0, x0, {}@PAGEOFF", label);

    // Push the string slice address onto the stack
    println!("    str x0, [sp, #-16]!");
}

// helper to emit code for array literal assignment
fn emit_array_assign(offset: u64, elements: &[Node]) {
    for (i, elem) in elements.iter().enumerate() {
        // evaluate element value
        gen_node(elem);
        // pop into x1
        println!("    ldr x1, [sp], #16");
        // compute element offset and store using register addressing
        let off_i = offset + (i as u64) * 8;
        println!("    mov x2, x29");
        println!("    sub x2, x2, #{}", off_i);
        println!("    str x1, [x2]");
    }
    // push dummy to maintain stack balance
    println!("    mov x0, #0");
    println!("    str x0, [sp, #-16]!");
}

// helper to recursively generate code for each node
fn gen_node(node: &Node) {
    match node {
        Node::Seq { first, second } => emit_seq(first, second),
        Node::Function { name, args, body } => emit_function(name, args, body),
        Node::Num { value } => push_imm(*value),
        Node::StringSlice { value } => emit_string(value),
        Node::Var { offset } => emit_var(*offset),
        Node::Call { name, args } => emit_call(name, args),
        Node::Syscall { name, args } => emit_syscall(name, args),
        Node::Return { expr } => emit_return(expr),
        Node::If {
            cond,
            then_stmt,
            else_stmt,
        } => emit_if(cond, then_stmt, else_stmt.as_deref()),
        Node::While { cond, body } => emit_while(cond, body),
        Node::For {
            init,
            cond,
            update,
            body,
        } => emit_for(init, cond, update, body),
        Node::ArrayAssign { offset, elements } => emit_array_assign(*offset, elements),
        Node::Assign { lhs, rhs } => emit_assign(lhs, rhs),
        Node::Add { lhs, rhs } => emit_binop("add", lhs, rhs),
        Node::Sub { lhs, rhs } => emit_binop("sub", lhs, rhs),
        Node::Mul { lhs, rhs } => emit_binop("mul", lhs, rhs),
        Node::Div { lhs, rhs } => emit_binop("sdiv", lhs, rhs),
        Node::Eq { lhs, rhs } => emit_cmp("eq", lhs, rhs),
        Node::Ne { lhs, rhs } => emit_cmp("ne", lhs, rhs),
        Node::Lt { lhs, rhs } => emit_cmp("lt", lhs, rhs),
        Node::Gt { lhs, rhs } => emit_cmp("gt", lhs, rhs),
        Node::Le { lhs, rhs } => emit_cmp("le", lhs, rhs),
        Node::Ge { lhs, rhs } => emit_cmp("ge", lhs, rhs),
        Node::Deref { expr } => emit_deref(expr),
        Node::Addr { expr } => emit_addr(expr),
    }
}

/// Generate full ARM64 assembly for the AST, including prologue and epilogue.
pub fn generate(node: &Node) {
    println!(".section __TEXT,__text");
    gen_node(node);
}
