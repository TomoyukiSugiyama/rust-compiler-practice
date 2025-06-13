// Test: Multiple variable assignments
// This test verifies that the compiler can handle chained variable assignments
// Expected return value: 3
//
// This file is not compatible with Rust because:
// 1. Variables must be declared with 'let' keyword
// 2. The return type of main() is not specified (should be () or Result<(), Box<dyn Error>>)
// 3. The variables 'foo' and 'bar' are not declared before use
// 4. Chained assignments are not allowed in Rust
fn main() {
    foo = bar = 2 + 1;
}
