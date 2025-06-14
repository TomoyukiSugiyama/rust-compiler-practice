// Test: Multiple nested for loops
// This test verifies that the compiler can handle:
// - Nested loops with separate iterators
// - Accumulation of values in outer and inner loops
// - Variable assignments and updates across loop scopes
// Expected return value: 60
//
// This file is not compatible with Rust because:
// 1. Variables must be declared with 'let' keyword
// 2. The return type of main() is not specified (should be () or Result<(), Box<dyn Error>>)
// 3. The return value of main() is not allowed in Rust (main should return unit type)
// 4. The variables 'sumi', 'sumj', 'i', and 'j' are not declared before use
// 5. For loop syntax is different in Rust (uses range or iterator)
//
fn main(){
    a=0;
    let sumi=0;
    let sumj=0;
    for ( i=0; i<5; i=i+1 ) {
        sumi=sumi+i;
        for ( j=0; j<5; j=j+1 ) {
            sumj=sumj+j;
        }
    }
    return sumi+sumj;
} 