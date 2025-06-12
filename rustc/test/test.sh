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
assert 12 'fn main(){12;}'
assert 1 'fn main(){1;}'
# addition
assert 3 'fn main(){1+2;}'
# subtraction
assert 1 'fn main(){4-3;}'
# addition and subtraction
assert 48 'fn main(){1+50-3;}'
assert 4 'fn main(){1+2-3+4;}'
# whitespace
assert 4 'fn main(){ 1  +2- 3+4 ; }'
# multiplication
assert 7 'fn main(){ 1 + 2 * 3 ;}'
# division
assert 4 'fn main(){ 6 - 6 / 3 ;}'
# parentheses
assert 9 'fn main(){ (1 + 2) * 3 ;}'
# unary
assert -3 'fn main(){-3;}'
assert -8 'fn main(){-(3+5);}'
assert -15 'fn main(){-3*+5;}'
# equality
assert 1 'fn main(){1==1;}'
assert 0 'fn main(){1==2;}'
assert 0 'fn main(){1!=1;}'
assert 1 'fn main(){1!=2;}'
# relational
assert 0 'fn main(){1<1;}'
assert 1 'fn main(){1<2;}'
assert 0 'fn main(){1>1;}'
assert 0 'fn main(){2<=1;}'
assert 1 'fn main(){1<=1;}'
assert 0 'fn main(){1>=2;}'
assert 1 'fn main(){1>=1;}'
# assignment
assert 3 'fn main(){foo=3;}'
# multiple assignment
assert 3 'fn main(){foo=bar=2+1;}'
# return statement
assert 3 'fn main(){return 4-1;}'
# assignment
assert 6 'fn main(){a=1; b=4; return a+b+1;}'
# if else statement
assert 3 'fn main(){if ( 1 == 1 ) return 3; else return 2;}'
assert 2 'fn main(){if ( 1 == 2 ) return 3; else return 2;}'
# while loop
assert 10 'fn main(){a=0; while ( a < 10 ) a=a+1; return a;}'
# for loop
assert 10 'fn main(){a=0; for ( i=0; i<10; i=i+1 ) a=a+1; return a;}'
# nested for loop
assert 4 'fn main(){a=0; b=1; for ( i=0; i<3; i=i+1 ) { a=a+1; b=b+1; } return b;}'
# function call
assert 5 'fn foo(){a=3;return a;} fn main(){b=foo(); return b+2;}'
# fibonacci
fib=$(cat ./test/assets/fibonacci-allow-warnings.rs)
assert 55 "$fib"
# dereference, address-of
assert 3 'fn main(){a=3;b=&a; return *b;}'


echo "OK"
