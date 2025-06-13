// Test: Greater than or equal comparison (true case)
// This test verifies that the compiler can handle greater than or equal comparisons
// and correctly evaluates to true when values are equal
// Expected return value: 1 (true)
//
// This file is not compatible with Rust because:
// 1. Comparison expressions cannot be used as standalone expressions
// 2. The return type of main() is not specified (should be () or Result<(), Box<dyn Error>>)
fn main() {
    1 >= 1;
}
