#!/bin/bash

set -eux

which patch
which cargo
which autoreconf
which libtool
which make
which mvn

# Build thirdparty libraries

pushd "./thirdparty/mecab/mecab"
./configure --prefix=$(cd .. && pwd)/tmpusr --libdir=$(cd .. && pwd)/tmpusr/lib
make
make install
popd

pushd "./thirdparty/mecab/mecab-ipadic"
./configure --with-charset=utf8 --prefix=$(cd .. && pwd)/tmpusr --with-mecab-config=../mecab/mecab-config
make
make install
popd

pushd "./thirdparty/kytea"
autoreconf -i
./configure --prefix=$(pwd)/tmpusr --libdir=$(pwd)/tmpusr/lib
make
make install
popd

# Build benches

pushd "./bench/vaporetto-bench"
cargo build --release
popd

pushd "./bench/kytea-bench"
LIBRARY_PATH=$(cd ../.. && pwd)/thirdparty/kytea/tmpusr/lib g++ -std=c++11 -O3 ./main.cc -I../../thirdparty/kytea/tmpusr/include -lkytea
popd

pushd "./bench/mecab-ipadic-2_7_0-bench"
LIBRARY_PATH=$(cd ../.. && pwd)/thirdparty/mecab/tmpusr/lib g++ -std=c++11 -O3 ./main.cc -I../../thirdparty/mecab/tmpusr/include -lmecab
popd

pushd "./bench/mecab-unidic-2_1_2-bench"
LIBRARY_PATH=$(cd ../.. && pwd)/thirdparty/mecab/tmpusr/lib g++ -std=c++11 -O3 ./main.cc -I../../thirdparty/mecab/tmpusr/include -lmecab
popd

pushd "./bench/mecab-unidic-3_1_0-bench"
LIBRARY_PATH=$(cd ../.. && pwd)/thirdparty/mecab/tmpusr/lib g++ -std=c++11 -O3 ./main.cc -I../../thirdparty/mecab/tmpusr/include -lmecab
popd

pushd "./bench/kuromoji-bench"
mvn compile
popd

pushd "./bench/lindera-bench"
cargo build --release
popd

pushd "./bench/sudachi-bench"
mvn compile
popd

pushd "./bench/sudachirs-bench"
cargo build --release
popd

pushd "./bench/rust-tinysegmenter-bench"
cargo build --release
popd

pushd "./bench/vibrato-bench"
cargo build --release
popd
