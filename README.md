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
- String literals with double quotes (`"..."`)
- System call support for writing to standard output without libc dependency

## Development Aids

- Debugging via external function calls (`debug1`, `debug2`, ...)
- Integration test framework with:
  - Return value verification
  - Standard output verification
  - Parallel test execution (default: 10 threads)

## Tests

### Unit Tests

```bash
% cd rustc
% cargo test
```

### Integration Tests

```bash
% cd rustc
% cargo run --bin test-runner [parallel_degree]
```

The integration tests can be run in parallel, with a default of 10 threads. You can specify a different number of threads as an argument to the test runner.

```bash
% cd rustc
% cargo run --bin test-runner
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/test-runner`
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
...............................F....F
Test                           Result Expected Got      Time
./test/assets/addition-and-subtraction.rs OK     48       0        2.1778935s
./test/assets/addition.rs      OK     3        0        1.20965025s
./test/assets/assign-multi.rs  OK     3        0        1.949310667s
./test/assets/assign-simple.rs OK     3        0        1.946465666s
./test/assets/assign-vars.rs   OK     6        0        1.932629125s
./test/assets/comments.rs      OK     10       0        1.785443875s
./test/assets/deref.rs         OK     3        0        1.773454667s
./test/assets/division.rs      OK     4        0        1.981395667s
./test/assets/eq-false.rs      OK     0        0        1.9426845s
./test/assets/eq-true.rs       OK     1        0        1.941518167s
./test/assets/fibonacci-allow-warnings.rs FAIL   54       0        1.773227375s
./test/assets/for-loop.rs      OK     10       0        1.935263583s
./test/assets/func-call.rs     OK     5        0        1.860954792s
    :
    :
./test/assets/string.rs        OK     0        0        1.698061625s
./test/assets/subtraction.rs   OK     1        0        1.790338625s
./test/assets/systemcall-write.rs FAIL   0        0        1.694768292s
./test/assets/unary-minus.rs   OK     -3       0        428.244708ms
./test/assets/unary-mixed.rs   OK     -15      0        1.943503833s
./test/assets/unary-neg-parens.rs OK     -8       0        1.408185709s
./test/assets/while-loop.rs    OK     10       0        1.927109166s
./test/assets/whitespace.rs    OK     4        0        815.637916ms

Failure details:
Test failed: ./test/assets/fibonacci-allow-warnings.rs
Expected return code: 54
Actual return code: 55
Test failed: ./test/assets/systemcall-write.rs
Expected output: "Hello, \nworld!!\n"
Actual output:   "Hello, \nworld!\n"
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

## Samples

This compiler can compile and run sample programs like the ones shown below. Each sample demonstrates practical examples of the compiler's features.

### Fibonacci

This sample calculates the Fibonacci number using a recursive function. It computes `fib(10)` and returns the result (55). This sample demonstrates the features of recursive function calls and control structures (if statements).

```bash
% cd rustc
% mkdir -p ./bin
% cargo run -- ./sample/fibonacci.rs > ./bin/fibonacci.s
% clang -arch arm64 -x assembler ./bin/fibonacci.s -o ./bin/fibonacci
% ./bin/fibonacci 
% echo $?
55
```

### Display Othello board

This sample displays the initial state of an Othello board using an array literal to represent piece codes and a string array for row labels. It demonstrates array literals, string array indexing, for loops (with initialization, condition, and increment expressions), nested loops, arithmetic operations and index access, conditional statements, and the `write` system call for output.

```bash
% cd rustc
% mkdir -p ./bin
% cargo run -- ./sample/display-othello-board.rs > ./bin/display-othello-board.s
% clang -arch arm64 -x assembler ./bin/display-othello-board.s -o ./bin/display-othello-board
% ./bin/display-othello-board
  A  B  C  D  E  F  G  H
1 ・ ・ ・ ・ ・ ・ ・ ・
2 ・ ・ ・ ・ ・ ・ ・ ・
3 ・ ・ ・ ・ ・ ・ ・ ・
4 ・ ・ ・ ○  ●  ・ ・ ・
5 ・ ・ ・ ●  ○  ・ ・ ・
6 ・ ・ ・ ・ ・ ・ ・ ・
7 ・ ・ ・ ・ ・ ・ ・ ・
8 ・ ・ ・ ・ ・ ・ ・ ・
```