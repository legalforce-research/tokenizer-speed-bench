#!/bin/bash

set -eux

which mvn

INPUT_DATA="./resources/wagahaiwa_nekodearu.txt"

# iter=0 is a warm-up to avoid unfair results due to lazy loading.
for i in $(seq 0 100)
do
    echo "iter" $i

    ./bench/vaporetto-bench/target/release/vaporetto-bench < $INPUT_DATA

    LD_LIBRARY_PATH=$PWD/thirdparty/kytea/tmpusr/lib ./bench/kytea-bench/a.out -model "./resources/jp-0.4.7-5.mod" < $INPUT_DATA

    LD_LIBRARY_PATH=$PWD/thirdparty/mecab/tmpusr/lib ./bench/mecab-ipadic-2_7_0-bench/a.out < $INPUT_DATA

    LD_LIBRARY_PATH=$PWD/thirdparty/mecab/tmpusr/lib ./bench/mecab-unidic-3_1_0-bench/a.out < $INPUT_DATA

    pushd ./bench/kuromoji-bench
    mvn exec:java -Dexec.mainClass=kuromoji_bench.App < ../../$INPUT_DATA
    popd

    ./bench/lindera-bench/target/release/lindera-bench < $INPUT_DATA

    pushd ./bench/sudachi-bench
    mvn exec:java -Dexec.mainClass=sudachi_bench.App < ../../$INPUT_DATA
    popd

    ./bench/sudachirs-bench/target/release/sudachirs-bench < $INPUT_DATA

    ./bench/rust-tinysegmenter-bench/target/release/rust-tinysegmenter-bench < $INPUT_DATA

    ./bench/vibrato-bench/target/release/vibrato-bench --dictname="ipadic-mecab-2_7_0" < $INPUT_DATA

    ./bench/vibrato-bench/target/release/vibrato-bench --dictname="unidic-cwj-3_1_0" < $INPUT_DATA
done
