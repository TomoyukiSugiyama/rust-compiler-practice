// Test: Fibonacci function with debug output
// This test verifies that the compiler can handle recursive function calls
// and debug output functionality
// Expected: Calculates fibonacci(10) and outputs the result using debug1
//
// This file is not compatible with Rust because:
// 1. The `debug1` function is not defined or imported
// 2. The `test` function is not marked with #[test] attribute
// 3. The main() function is missing
fn fib(n: i32) -> i32 {
    if (n <= 1) {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

fn test() {
    let res = fib(10);
    debug1(res);
}
