// Test: Multiple variable assignments and arithmetic
// This test verifies that the compiler can handle:
// - Multiple variable assignments
// - Arithmetic operations with variables
// - Return statements with complex expressions
// Expected return value: 6
//
// This file is not compatible with Rust because:
// 1. Variables must be declared with 'let' keyword
// 2. The return type of main() is not specified (should be () or Result<(), Box<dyn Error>>)
// 3. The return value of main() is not allowed in Rust (main should return unit type)
// 4. The variables 'a' and 'b' are not declared before use
fn main() {
    a = 1;
    b = 4;
    return a + b + 1;
}
