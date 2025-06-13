// Test: Fibonacci function implementation
// This test verifies that the compiler can handle:
// - Recursive function calls
// - Conditional statements
// - Return statements
// - Arithmetic operations
// Expected: Calculates fibonacci(10)
//
// This file is not compatible with Rust because:
// 1. The return type of functions is not specified
// 2. The return type of main() is not specified (should be () or Result<(), Box<dyn Error>>)
// 3. The return value of main() is not allowed in Rust (main should return unit type)
// 4. Parentheses around if condition are not needed in Rust
fn fib(n: i32) -> i32 {
    // Note: Rust warns about parentheses around the `if` condition, but our compiler ignores these warnings.
    if (n <= 1) {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

fn main() {
    fib(10);
}
