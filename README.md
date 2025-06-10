# Setup

```bash
$ mkdir -p rustc/bin
```

# Test

```bash
# unit test
$ cd rustc
$ cargo test
# integration test
$ cd rustc
$ ./test/test.sh
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rustc 'a=0; b=1; for ( i=0; i<3; i=i+1 ) { a=a+1; b=b+1; } return b;'`
a=0; b=1; for ( i=0; i<3; i=i+1 ) { a=a+1; b=b+1; } return b; => 4
OK

```

# Function call test

## 1. Generate `test-foo.s`

```bash
% cd rustc
# debug function call
% cargo run -- 'fn test(){debug2(1, 2);}' > bin/test-debug.s
# debug fibonacci
$ cargo run -- 'fn fib(n) { if (n <= 1) { return n; } return fib(n - 1) + fib(n - 2); } fn test() { res = fib(10);debug1(res); return res; }' > bin/test-debug.s
```


## 2. Build and run the integration test

```sh
% cd rustc
% cargo run --manifest-path test/function-call/Cargo.toml
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `rustc/test/function-call/target/debug/foo`
foo
```