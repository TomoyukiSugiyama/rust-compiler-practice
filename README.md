

```bash
$ cargo run -- 12 > ./bin/test-arm64.s
$ clang -arch arm64 -x assembler  ./bin/test-arm64.s -o ./bin/test
$ ./bin/test
$ echo $?
12
```