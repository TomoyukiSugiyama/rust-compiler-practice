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

assert 12 '12;'
assert 1 '1;'
assert 3 '1+2;'
assert 1 '4-3;'
assert 48 '1+50-3;'
assert 4 '1+2-3+4;'
assert 4 ' 1  +2- 3+4 ; '
assert 7 ' 1 + 2 * 3 ;'
assert 4 ' 6 - 6 / 3 ;'
assert 9 ' (1 + 2) * 3 ;'
assert -3 '-3;'
assert -8 '-(3+5);'
assert -15 '-3*+5;'
assert 1 '1==1;'
assert 0 '1==2;'
assert 0 '1!=1;'
assert 1 '1!=2;'
assert 0 '1<1;'
assert 1 '1<2;'
assert 0 '1>1;'
assert 0 '2<=1;'
assert 1 '1<=1;'
assert 0 '1>=2;'
assert 1 '1>=1;'
assert 3 'foo=3;'
assert 3 'foo=bar=2+1;'

echo "OK"
