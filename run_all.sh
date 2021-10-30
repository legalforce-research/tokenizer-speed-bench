#!/bin/bash

set -eux

which node
which mvn
which python3

INPUT_DATA="./resources/wagahaiwa_nekodearu.txt"

for i in 0 1 2 3 4 5 6 7 8 9
do
    for j in 0 1 2 3 4 5 6 7 8 9
    do
        echo "iter" $i $j

        ./bench/vaporetto-bench/target/release/vaporetto-bench < $INPUT_DATA

        node ./bench/vaporetto-wasm-bench/index.js < $INPUT_DATA

        LD_LIBRARY_PATH=$PWD/thirdparty/kytea/tmpusr/lib ./bench/kytea-bench/a.out -model "./resources/jp-0.4.7-6.mod" < $INPUT_DATA

        LD_LIBRARY_PATH=$PWD/thirdparty/mecab/tmpusr/lib ./bench/mecab-bench/a.out < $INPUT_DATA

        node ./bench/tinysegmenter-bench/index.js < $INPUT_DATA

        pushd ./bench/kuromoji-bench
        mvn exec:java -Dexec.mainClass=kuromoji_bench.App < ../../$INPUT_DATA
        popd

        ./bench/lindera-bench/target/release/lindera-bench < $INPUT_DATA

        pushd ./bench/sudachi-bench
        mvn exec:java -Dexec.mainClass=sudachi_bench.App < ../../$INPUT_DATA
        popd

        python3 ./bench/janome-bench/main.py < $INPUT_DATA

        ./bench/sudachirs-bench/target/release/sudachirs-bench < $INPUT_DATA
    done
done
