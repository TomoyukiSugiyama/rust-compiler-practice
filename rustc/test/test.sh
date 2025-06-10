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

assert 12 'fn main(){12;}'
assert 1 'fn main(){1;}'
assert 3 'fn main(){1+2;}'
assert 1 'fn main(){4-3;}'
assert 48 'fn main(){1+50-3;}'
assert 4 'fn main(){1+2-3+4;}'
assert 4 'fn main(){ 1  +2- 3+4 ; }'
assert 7 'fn main(){ 1 + 2 * 3 ;}'
assert 4 'fn main(){ 6 - 6 / 3 ;}'
assert 9 'fn main(){ (1 + 2) * 3 ;}'
assert -3 'fn main(){-3;}'
assert -8 'fn main(){-(3+5);}'
assert -15 'fn main(){-3*+5;}'
assert 1 'fn main(){1==1;}'
assert 0 'fn main(){1==2;}'
assert 0 'fn main(){1!=1;}'
assert 1 'fn main(){1!=2;}'
assert 0 'fn main(){1<1;}'
assert 1 'fn main(){1<2;}'
assert 0 'fn main(){1>1;}'
assert 0 'fn main(){2<=1;}'
assert 1 'fn main(){1<=1;}'
assert 0 'fn main(){1>=2;}'
assert 1 'fn main(){1>=1;}'
assert 3 'fn main(){foo=3;}'
assert 3 'fn main(){foo=bar=2+1;}'
assert 3 'fn main(){return 4-1;}'
assert 6 'fn main(){a=1; b=4; return a+b+1;}'
assert 3 'fn main(){if ( 1 == 1 ) return 3; else return 2;}'
assert 2 'fn main(){if ( 1 == 2 ) return 3; else return 2;}'
assert 10 'fn main(){a=0; while ( a < 10 ) a=a+1; return a;}'
assert 10 'fn main(){a=0; for ( i=0; i<10; i=i+1 ) a=a+1; return a;}'
assert 4 'fn main(){a=0; b=1; for ( i=0; i<3; i=i+1 ) { a=a+1; b=b+1; } return b;}'
assert 5 'fn foo(){a=3;return a;} fn main(){b=foo(); return b+2;}'
echo "OK"
