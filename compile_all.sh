#!/bin/bash

set -eux

which patch
which cargo
which autoreconf
which libtool
which make
which mvn

pushd "./bench/vaporetto-bench"
cargo build --release
popd

pushd "./thirdparty/kytea"
autoreconf -i
./configure --prefix=$(pwd)/tmpusr
make
make install
popd
pushd "./bench/kytea-bench"
g++ -std=c++11 -O2 ./main.cc -I../../thirdparty/kytea/src/include/ ../../thirdparty/kytea/tmpusr/lib/libkytea.a
popd

pushd "./thirdparty/mecab/mecab"
./configure --prefix=$(cd .. && pwd)/tmpusr
make
make install
popd
pushd "./thirdparty/mecab/mecab-ipadic"
./configure --with-charset=utf8 --prefix=$(cd .. && pwd)/tmpusr --with-mecab-config=../mecab/mecab-config
make
make install
popd
pushd "./bench/mecab-bench"
g++ -std=c++11 -O2 ./main.cc -I../../thirdparty/mecab/tmpusr/include ../../thirdparty/mecab/tmpusr/lib/libmecab.a -liconv
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
