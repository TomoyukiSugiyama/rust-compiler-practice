// Test: Return statement with arithmetic expression
// This test verifies that the compiler can handle return statements
// with arithmetic expressions
// Expected return value: 3
//
// This file is not compatible with Rust because:
// 1. The return type of main() is not specified (should be () or Result<(), Box<dyn Error>>)
// 2. The return value of main() is not allowed in Rust (main should return unit type)
fn main() {
    return 4 - 1;
}
