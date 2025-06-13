// Test: Greater than comparison (true case)
// This test verifies that the compiler can handle greater than comparisons
// and correctly evaluates to true when left value is larger
// Expected return value: 1 (true)
//
// This file is not compatible with Rust because:
// 1. Comparison expressions cannot be used as standalone expressions
// 2. The return type of main() is not specified (should be () or Result<(), Box<dyn Error>>)
fn main() {
    2 > 1;
}
