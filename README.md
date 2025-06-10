# setup

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

# Function Call Test

## 1. Generate `test-foo.s`

```bash
% cd rustc
% cargo run -- 'foo();' > bin/test-foo.s
```

## 2. Edit `test-foo.s`

```diff
< .globl _main
< _main:
> .globl _test
> _test:
```

## 3. Build and run the integration test

```sh
% cd rustc
% cargo run --manifest-path test/function-call/Cargo.toml
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `rustc/test/function-call/target/debug/foo`
foo
```