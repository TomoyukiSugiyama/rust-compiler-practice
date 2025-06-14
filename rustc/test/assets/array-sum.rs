// Test: Sum elements of an array using for loop
// This test verifies that the compiler can handle:
// - Array literal syntax
// - Element indexing with []
// - Variable declaration and initialization
// - For loop initialization, condition, and increment
// - Variable assignment and accumulation within loops
// Expected return value: 15
//
// This file is not compatible with Rust because:
// 1. Variables must be declared with 'let' keyword
// 2. The return type of main() is not specified (should be () or Result<(), Box<dyn Error>>)
// 3. The return value of main() is not allowed in Rust (main should return unit type)
// 4. The loop variable 'i' is not declared before use
// 5. For loop syntax is different in Rust (uses range or iterator)

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let sum = 0;
    for ( i=0; i<5; i=i+1 ) {
        sum = sum + arr[i];
    }
    return sum;
}
