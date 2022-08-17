#!/bin/bash

set -eux

which wget
which gunzip
which unzip

pushd "./resources"

wget "http://www.phontron.com/kytea/download/model/jp-0.4.7-5.mod.gz" -O "./jp-0.4.7-5.mod.gz"
rm -f "./jp-f.4.7-5.mod"
gunzip "./jp-0.4.7-5.mod.gz"

wget "https://clrd.ninjal.ac.jp/unidic_archive/cwj/2.1.2/unidic-mecab-2.1.2_bin.zip" -O "./unidic-mecab-2.1.2_bin.zip"
rm -rf "./unidic-mecab-2.1.2_bin"
unzip "./unidic-mecab-2.1.2_bin.zip"

popd

pushd "./bench/sudachi-bench"
wget "http://sudachi.s3-website-ap-northeast-1.amazonaws.com/sudachidict/sudachi-dictionary-20210802-core.zip" -O "./sudachi-dictionary-20210802-core.zip"
rm -rf "./sudachi-dictionary-20210802"
unzip "./sudachi-dictionary-20210802-core.zip"
popd

pushd "./thirdparty/vibrato"
./scripts/prepare_ipadic-mecab-2_7_0.sh
./scripts/prepare_unidic-mecab-2_1_2.sh
popd

mv "./thirdparty/vibrato/resources_ipadic-mecab-2_7_0" "./bench/vibrato-bench/"
mv "./thirdparty/vibrato/resources_unidic-mecab-2_1_2" "./bench/vibrato-bench/"
