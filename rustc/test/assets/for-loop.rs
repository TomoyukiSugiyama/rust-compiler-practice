// Test: For loop with counter
// This test verifies that the compiler can handle:
// - For loop initialization
// - Loop condition
// - Loop increment
// - Variable assignments within loops
// Expected return value: 10
//
// This file is not compatible with Rust because:
// 1. Variables must be declared with 'let' keyword
// 2. The return type of main() is not specified (should be () or Result<(), Box<dyn Error>>)
// 3. The return value of main() is not allowed in Rust (main should return unit type)
// 4. The variables 'a' and 'i' are not declared before use
// 5. For loop syntax is different in Rust (uses range or iterator)
fn main(){
    a=0;
    for ( i=0; i<10; i=i+1 ) {
        a=a+1;
    }
    return a;
} 