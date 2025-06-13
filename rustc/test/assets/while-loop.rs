// Test: While loop with variable increment
// This test verifies that the compiler can handle while loops
// and variable assignments within loops
// Expected return value: 10
//
// This file is not compatible with Rust because:
// 1. Variables must be declared with 'let' keyword
// 2. The return type of main() is not specified (should be () or Result<(), Box<dyn Error>>)
// 3. The return value of main() is not allowed in Rust (main should return unit type)
fn main() {
    a = 0;
    while (a < 10) {
        a = a + 1;
    }
    return a;
}
