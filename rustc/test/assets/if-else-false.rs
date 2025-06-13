// Test: If-else statement with false condition
// This test verifies that the compiler can handle if-else statements
// and correctly evaluates the false branch
// Expected return value: 2
//
// This file is not compatible with Rust because:
// 1. The return type of main() is not specified (should be () or Result<(), Box<dyn Error>>)
// 2. The return value of main() is not allowed in Rust (main should return unit type)
fn main() {
    if (1 == 2) {
        return 3;
    } else {
        return 2;
    }
}
