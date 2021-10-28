#!/bin/bash

set -eux

which wget
which gunzip
which unzip
which tar

mkdir -p "./resources"
pushd "./resources"
wget "http://www.phontron.com/kytea/download/model/jp-0.4.7-6.mod.gz" -O "./jp-0.4.7-6.mod.gz"
rm -f "./jp-f.4.7-6.mod"
gunzip "./jp-0.4.7-6.mod.gz"
wget "http://www.phontron.com/kftt/download/kftt-data-1.0.tar.gz" -O "./kftt-data-1.0.tar.gz"
rm -rf "./kftt-data-1.0"
tar xf "./kftt-data-1.0.tar.gz"
popd
pushd "./bench/sudachi-bench"
wget "http://sudachi.s3-website-ap-northeast-1.amazonaws.com/sudachidict/sudachi-dictionary-20210802-core.zip" -O "./sudachi-dictionary-20210802-core.zip"
rm -rf "./sudachi-dictionary-20210802"
unzip "./sudachi-dictionary-20210802-core.zip"
popd
pushd "./thirdparty/sudachi.rs"
"./fetch_dictionary.sh"
popd
