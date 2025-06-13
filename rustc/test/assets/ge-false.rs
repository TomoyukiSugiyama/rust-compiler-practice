// Test: Greater than or equal comparison (false case)
// This test verifies that the compiler can handle greater than or equal comparisons
// and correctly evaluates to false when left value is smaller
// Expected return value: 0 (false)
//
// This file is not compatible with Rust because:
// 1. Comparison expressions cannot be used as standalone expressions
// 2. The return type of main() is not specified (should be () or Result<(), Box<dyn Error>>)
fn main() {
    1 >= 2;
}
