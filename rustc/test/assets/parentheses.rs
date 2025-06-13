// Test: Parentheses in expressions
// This test verifies that the compiler correctly handles parentheses
// to override operator precedence
// Expected return value: 9
//
// This file is not compatible with Rust because:
// 1. Arithmetic expressions cannot be used as standalone expressions
// 2. The return type of main() is not specified (should be () or Result<(), Box<dyn Error>>)
fn main() {
    (1 + 2) * 3;
}
