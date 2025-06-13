// This file is used to test the fibonacci function.
fn fib(n: i32) -> i32 {
    // Note: Rust warns about parentheses around the `if` condition, but our compiler ignores these warnings.
    if (n <= 1) {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

fn main() {
    let res = fib(10);
    return res;
}
