// Test: String literal handling
// This test verifies that the compiler can handle string literals
// Expected return value: 0
//
// This file is not compatible with Rust because:
// 1. String literals cannot be used as standalone expressions
// 2. The return type of main() is not specified (should be () or Result<(), Box<dyn Error>>)
fn main() {
    "Hello, world!";
    return 0;
}
