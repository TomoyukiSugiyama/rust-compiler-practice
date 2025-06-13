// Test: Nested variable updates in loop
// This test verifies that the compiler can handle:
// - Multiple variable assignments
// - Variable updates within loops
// - Multiple statements in loop body
// Expected return value: 4
//
// This file is not compatible with Rust because:
// 1. Variables must be declared with 'let' keyword
// 2. The return type of main() is not specified (should be () or Result<(), Box<dyn Error>>)
// 3. The return value of main() is not allowed in Rust (main should return unit type)
// 4. The variables 'a', 'b', and 'i' are not declared before use
// 5. For loop syntax is different in Rust (uses range or iterator)
fn main(){
    a=0;
    b=1;
    for ( i=0; i<3; i=i+1 ) {
            a=a+1;
            b=b+1;
    }
    return b;
} 