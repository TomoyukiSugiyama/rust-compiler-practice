#!/bin/zsh

assert() {
    expected=$1
    input=$2

    cargo run -- $input > ./bin/test-arm64.s
    clang -arch arm64 -x assembler  ./bin/test-arm64.s -o ./bin/test

    ./bin/test
    actual="$?"

    # Convert actual exit code to signed 8-bit
    if [ $actual -gt 127 ]; then
        actual=$((actual - 256))
    fi

    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual"
    else
        echo "$input => $expected expected, but got $actual"
        exit 1
    fi
}

# number
assert 12 "./test/assets/number.rs"
# # addition
assert 3 "./test/assets/addition.rs"
# # subtraction
assert 1 "./test/assets/subtraction.rs"
# # addition and subtraction
assert 48 "./test/assets/addition-and-subtraction.rs"
# # whitespace
assert 4 "./test/assets/whitespace.rs"
# # multiplication
assert 7 "./test/assets/multiplication.rs"
# # division
assert 4 "./test/assets/division.rs"
# # parentheses
assert 9 "./test/assets/parentheses.rs"
# # unary
assert -3 "./test/assets/unary-minus.rs"
assert -8 "./test/assets/unary-neg-parens.rs"
assert -15 "./test/assets/unary-mixed.rs"
# # equality
assert 1 "./test/assets/eq-true.rs"
assert 0 "./test/assets/eq-false.rs"
# # relational
assert 0 "./test/assets/lt-false.rs"
assert 1 "./test/assets/lt-true.rs"
assert 0 "./test/assets/gt-false.rs"
assert 1 "./test/assets/gt-true.rs"
assert 0 "./test/assets/le-false.rs"
assert 1 "./test/assets/le-true.rs"
assert 0 "./test/assets/ge-false.rs"
assert 1 "./test/assets/ge-true.rs"
# # assignment
assert 3 "./test/assets/assign-simple.rs"
# # multiple assignment
assert 3 "./test/assets/assign-multi.rs"
# # return statement
assert 3 "./test/assets/return-stmt.rs"
# # assignment
assert 6 "./test/assets/assign-vars.rs"
# # if else statement
assert 3 "./test/assets/if-else-true.rs"
assert 2 "./test/assets/if-else-false.rs"
# # while loop
assert 10 "./test/assets/while-loop.rs"
# # for loop
assert 10 "./test/assets/for-loop.rs"
# # nested for loop
assert 4 "./test/assets/nested-loop.rs"
# # function call
assert 5 "./test/assets/func-call.rs"
# fibonacci
assert 55 "./test/assets/fibonacci-allow-warnings.rs"
# # dereference, address-of
assert 3 "./test/assets/deref.rs"
# # local variable
assert 10 "./test/assets/local-var.rs"
# # comment out
assert 10 "./test/assets/comments.rs"

echo "OK"
