# setup

```bash
$ mkdir -p rustc/bin
```

# test

```bash
$ cd rustc
$ ./test/test.sh
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rustc 'a=0; b=1; for ( i=0; i<3; i=i+1 ) { a=a+1; b=b+1; } return b;'`
a=0; b=1; for ( i=0; i<3; i=i+1 ) { a=a+1; b=b+1; } return b; => 4
OK

```

# cross compile

## 1. Generate `test-foo.s`

```bash
$ cd rustc
$ cargo run -- 'foo();' > rustc/bin/test-foo.s
```

## 2. Edit `test-foo.s`

```
< .globl _main
< _main:
> .globl _test
> _test:
```

## 3. Build and run the cross-compile example

```bash
$ cargo run --manifest-path rustc/test/cross-compile/Cargo.toml
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/foo`
 foo bar
```