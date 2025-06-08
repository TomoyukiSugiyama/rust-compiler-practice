use std::env;

fn main() {
    let exp = env::args().nth(1).expect("Usage: program <exp>");

    // Parse expression into numbers and operators (supports multiple + and -)
    let mut nums = Vec::new();
    let mut ops = Vec::new();
    let mut curr: u64 = 0;
    for c in exp.chars() {
        if c.is_ascii_digit() {
            let d = c.to_digit(10).unwrap() as u64;
            curr = curr.wrapping_mul(10).wrapping_add(d);
        } else if c == '+' || c == '-' {
            nums.push(curr);
            ops.push(c);
            curr = 0;
        } else {
            panic!("Invalid character in expression: {}", c);
        }
    }
    nums.push(curr);

    println!(".section __TEXT,__text");
    println!(".globl _main");
    println!("_main:");
    // Move first operand into x0
    println!("    mov x0, #{}", nums[0]);
    // Apply each operator in sequence
    for (i, &op) in ops.iter().enumerate() {
        let rhs = nums[i + 1];
        match op {
            '+' => println!("    add x0, x0, #{}", rhs),
            '-' => println!("    sub x0, x0, #{}", rhs),
            _ => unreachable!(),
        }
    }
    println!("    ret");
}
