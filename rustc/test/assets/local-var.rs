// Test: Local variable declaration and usage
// This test verifies that the compiler can handle:
// - Variable declarations with 'let' keyword
// - Variable assignments
// - Return statements with variables
// Expected return value: 10
//
// This file is not compatible with Rust because:
// 1. The return type of main() is not specified (should be () or Result<(), Box<dyn Error>>)
// 2. The return value of main() is not allowed in Rust (main should return unit type)
fn main() {
    let num = 10;
    return num;
}
