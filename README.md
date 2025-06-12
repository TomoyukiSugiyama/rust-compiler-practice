# rust-compiler-practice

A simple Rust-like compiler implemented in Rust for learning compiler construction and Rust internals. It aims to compile pseudo-code close to Rust syntax, supporting various language features.

## Features

- Generation of ARM64 assembly
- Integer literals and arithmetic operations: +, -, *, /
- Unary operators: + and -
- Parentheses for grouping
- Comparison operators: ==, !=, <, <=, >, >=
- Variable assignment: basic and chained
- Local variables with `let`
- Return statements
- Comments: single-line (`//`) and multi-line (`/* ... */`)
- Control flow: `if-else`, `for` and `while` loops
- Function definitions and calls, including recursion and parameters
- Memory operations: references (`&`) and dereferences (`*`)

## Development Aids

- Debugging via external function calls (`debug1`, `debug2`, ...)

## Tests

### Unit Tests

```bash
% cd rustc
% cargo test
```

### Integration Tests

```bash
% cd rustc
% cargo run --bin test-runner
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rustc ./test/assets/number.rs`
./test/assets/number.rs => 12

  :
  :
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rustc ./test/assets/local-var.rs`
./test/assets/local-var.rs => 10
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rustc ./test/assets/comments.rs`
./test/assets/comments.rs => 10
OK
```

## Debugging via Function Calls

Since our compiler currently does not support printing variable values to standard output, we use function calls to Rust's `println!` macro as a workaround.

How it works:
- In `test/assets/fibonacci-debug.rs`, we embed a call to `debug1` and implement a `test` function instead of `main`.
- In the Rust `main` function, we call `test()` inside an `unsafe` block, which invokes `debug1` to execute `println!`.

### 1. Create the Fibonacci source

Create `test/assets/fibonacci-debug.rs` with:
```rust
fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

fn test() {
    let res = fib(10);
    debug1(res);
}
```

### 2. Generate assembly

From the `rustc` directory, run:
```bash
% cargo run -- ./test/assets/fibonacci-debug.rs > ./bin/test-debug.s
```

This produces `bin/test-debug.s` containing arm64 assembly for the test.

### 3. Run the integration test

Navigate to the function-call test project and run:
```bash
% cargo run --manifest-path test/function-call/Cargo.toml
```

You should see:
```
x = 55
```
