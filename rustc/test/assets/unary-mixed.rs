// Test: Mixed unary operators
// This test verifies that the compiler can handle mixed unary operators
// (negative and positive) in expressions
// Expected return value: -15
//
// This file is not compatible with Rust because:
// 1. Arithmetic expressions cannot be used as standalone expressions
// 2. The return type of main() is not specified (should be () or Result<(), Box<dyn Error>>)
fn main(){
    -3*+5;
} 