// Test: Multiple independent if statements
// This test verifies that the compiler can handle multiple sequential if statements
// and correctly updates variables based on each condition
// Expected return value: 2
//
// This file is not compatible with Rust because:
// 1. The return type of main() is not specified (should be () or Result<(), Box<dyn Error>>)
// 2. The return value of main() is not allowed in Rust (main should return unit type)

fn main() {
    let res = 0;
    if (1 == 2) {
        res = res + 1;
    }
    if (1 == 1) {
        res = res + 2;
    }
    return res;
}
