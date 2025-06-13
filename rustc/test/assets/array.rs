// Test: Array literal and indexing
// This test verifies that the compiler can handle:
// - Array literal syntax
// - Element indexing with []
// Expected return value: 3
//
// This file is not compatible with Rust because:
// 1. The return type of main() is not specified (should be () or Result<(), Box<dyn Error>>)
// 2. The return value of main() is not allowed in Rust (main should return unit type)

fn main() {
    let arr = [1, 2, 3, 4, 5];
    return arr[2];
}
