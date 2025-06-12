## Setup

```bash
$ mkdir -p rustc/bin
```

## Tests

### Unit Tests

```bash
cd rustc
cargo test
```

### Integration Tests

```bash
cd rustc
% ./test/test.sh 
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

## Function Call Test

This test verifies that our compiler correctly handles function calls by compiling a recursive Fibonacci function into assembly and running it in an integration test.

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
cargo run -- ./test/assets/fibonacci-debug.rs > ./bin/test-debug.s
```

This produces `bin/test-debug.s` containing arm64 assembly for the test.

### 3. Run the integration test

Navigate to the function-call test project and run:
```bash
cd test/function-call
cargo run
```

You should see:
```
x = 55
```
