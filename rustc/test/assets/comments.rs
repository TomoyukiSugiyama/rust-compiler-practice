// Test: Comment handling
// This test verifies that the compiler can handle various types of comments:
// - Single line comments (//)
// - Multi-line comments (/* */)
// - Japanese comments
// Expected return value: 10
//
// This file is not compatible with Rust because:
// 1. The return type of main() is not specified (should be () or Result<(), Box<dyn Error>>)
// 2. The return value of main() is not allowed in Rust (main should return unit type)
// これは、comment outのテストです。
fn main() {
    // 関数内のコメントアウト
    /*
    これは、
    複数行の
    コメントアウトです。
    */
    let num = 10;
    return num;
}
