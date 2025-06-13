// Test: System call write function
// This test verifies that the compiler can handle system call write function
// Expected output: "Hello, \nworld!\n"
//
// This file is not compatible with Rust because:
// 1. The `write` function is not defined or imported
// 2. In Rust, we need to use std::io::Write trait and implement proper error handling
fn main() {
    let str = "Hello, \nworld!\n";
    write(str);
}
