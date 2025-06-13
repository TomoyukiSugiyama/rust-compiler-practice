// Test: Function call and return value handling
// This test verifies that the compiler can handle:
// - Function definitions
// - Function calls
// - Return value assignments
// - Arithmetic operations with return values
// Expected return value: 5
//
// This file is not compatible with Rust because:
// 1. Variables must be declared with 'let' keyword
// 2. The return type of functions is not specified
// 3. The return type of main() is not specified (should be () or Result<(), Box<dyn Error>>)
// 4. The return value of main() is not allowed in Rust (main should return unit type)
fn foo() {
    a = 3;
    return a;
}
fn main() {
    b = foo();
    return b + 2;
}
