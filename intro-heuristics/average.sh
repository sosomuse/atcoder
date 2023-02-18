#!/usr/bin/env bash

i=0
for text in `cat seed.txt`
do
    if [ ! -d "result" ]; then
        mkdir result
    fi
    python3 generator.py $text > result/$i.in
    cargo run --bin a < result/$i.in > result/$i.out
    python3 tester.py result/$i.in result/$i.out > result/$i.log
    i=$((i+1))
done

