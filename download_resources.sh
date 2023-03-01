#!/bin/bash

set -eux

which wget
which gunzip
which unzip

pushd "./resources"

wget "http://www.phontron.com/kytea/download/model/jp-0.4.7-5.mod.gz" -O "./jp-0.4.7-5.mod.gz"
rm -f "./jp-f.4.7-5.mod"
gunzip "./jp-0.4.7-5.mod.gz"

wget "https://ccd.ninjal.ac.jp/unidic_archive/cwj/3.1.1/unidic-cwj-3.1.1.zip" -O "./unidic-cwj-3.1.1.zip"
rm -rf "./unidic-cwj-3.1.1"
unzip "./unidic-cwj-3.1.1.zip"

popd

pushd "./bench/sudachi-bench"
wget "http://sudachi.s3-website-ap-northeast-1.amazonaws.com/sudachidict/sudachi-dictionary-20210802-core.zip" -O "./sudachi-dictionary-20210802-core.zip"
rm -rf "./sudachi-dictionary-20210802"
unzip "./sudachi-dictionary-20210802-core.zip"
popd

pushd "./bench/vibrato-bench"
wget https://github.com/daac-tools/vibrato/releases/download/v0.5.0/ipadic-mecab-2_7_0.tar.xz
tar -xf ipadic-mecab-2_7_0.tar.xz
wget https://github.com/daac-tools/vibrato/releases/download/v0.5.0/unidic-cwj-3_1_1.tar.xz
tar -xf unidic-cwj-3_1_1.tar.xz
wget https://github.com/daac-tools/vibrato/releases/download/v0.5.0/unidic-cwj-3_1_1+compact.tar.xz
tar -xf unidic-cwj-3_1_1+compact.tar.xz
wget https://github.com/daac-tools/vibrato/releases/download/v0.5.0/unidic-cwj-3_1_1+compact-dual.tar.xz
tar -xf unidic-cwj-3_1_1+compact-dual.tar.xz
popd
