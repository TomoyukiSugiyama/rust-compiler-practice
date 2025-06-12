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
     Running `target/debug/test-runner`
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
...................................
Test                           Result Expected Got      Time
./test/assets/addition-and-subtraction.rs OK     48       48       712.229042ms
./test/assets/addition.rs      OK     3        3        1.338276125s
./test/assets/assign-multi.rs  OK     3        3        2.105728625s
./test/assets/assign-simple.rs OK     3        3        2.095722791s
./test/assets/assign-vars.rs   OK     6        6        2.115379958s
./test/assets/comments.rs      OK     10       10       2.035201333s
./test/assets/deref.rs         OK     3        3        2.053940625s
./test/assets/division.rs      OK     4        4        2.153181333s
./test/assets/eq-false.rs      OK     0        0        2.057194084s
./test/assets/eq-true.rs       OK     1        1        2.055929458s
./test/assets/fibonacci-allow-warnings.rs OK     55       55       2.07738125s
./test/assets/for-loop.rs      OK     10       10       2.125089541s
./test/assets/func-call.rs     OK     5        5        2.088334375s
./test/assets/ge-false.rs      OK     0        0        2.070356416s
./test/assets/ge-true.rs       OK     1        1        2.085557875s
./test/assets/gt-false.rs      OK     0        0        2.066016542s
./test/assets/gt-true.rs       OK     1        1        2.066933875s
./test/assets/if-else-false.rs OK     2        2        2.109805167s
./test/assets/if-else-true.rs  OK     3        3        2.112992625s
./test/assets/le-false.rs      OK     0        0        2.060584375s
./test/assets/le-true.rs       OK     1        1        2.066830375s
./test/assets/local-var.rs     OK     10       10       2.042662417s
./test/assets/lt-false.rs      OK     0        0        2.067007875s
./test/assets/lt-true.rs       OK     1        1        2.05786825s
./test/assets/multiplication.rs OK     7        7        1.536999542s
./test/assets/nested-loop.rs   OK     4        4        2.109025458s
./test/assets/number.rs        OK     12       12       1.742669458s
./test/assets/parentheses.rs   OK     9        9        508.070417ms
./test/assets/return-stmt.rs   OK     3        3        2.116414584s
./test/assets/subtraction.rs   OK     1        1        2.360452209s
./test/assets/unary-minus.rs   OK     -3       -3       920.25725ms
./test/assets/unary-mixed.rs   OK     -15      -15      2.056690542s
./test/assets/unary-neg-parens.rs OK     -8       -8       1.124811875s
./test/assets/while-loop.rs    OK     10       10       2.132702334s
./test/assets/whitespace.rs    OK     4        4        1.946823125s
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
