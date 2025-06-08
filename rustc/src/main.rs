use std::env;

fn main() {
    let num_str = env::args().nth(1).expect("Usage: program <number>");
    let num = num_str
        .parse::<u64>()
        .expect("Argument must be a positive integer");

    println!(".section __TEXT,__text");
    println!(".globl _main");
    println!("_main:");
    println!("    mov x0, #{}", num);
    println!("    ret");
}
