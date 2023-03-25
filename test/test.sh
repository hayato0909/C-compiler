#!/bin/bash
assert() {
    expected="$1"
    input="$2"

    ./../target/debug/compiler "$input" > ../tmp/tmp.s
    cc -o tmp ../tmp/tmp.s
    ./tmp
    actual="$?"
    rm tmp

    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual"
    else
        echo "$input => $expected expected, but got $actual"
        exit 1
    fi
}

cargo build 
assert 0 "return 0;"
assert 42 "return 42;"
assert 21 "return 5+20-4;"
assert 41 "return 12 + 34 - 5;"
assert 47 "return 5+6*7;"
assert 15 "return 5*(9-6);"
assert 4 "return (3+5)/2;"

assert 10 "return -10+20;"
assert 9 "return +28-19;"
#assert 10 "- -10"
#assert 10 "- - +10"

assert 0 "return 0==1;"
assert 1 "return 41==41;"
assert 1 "return 0!=1;"
assert 0 "return 41!=41;"

assert 1 "return 0<1;"
assert 0 "return 1<1;"
assert 0 "return 2<1;"
assert 1 "return 0<=1;"
assert 1 "return 1<=1;"
assert 0 "return 2<=1;"

assert 1 "1>0;"
assert 0 "1>1;"
assert 0 "1>2;"
assert 1 "1>=0;"
assert 1 "1>=1;"
assert 0 "1>=2;"

echo OK
