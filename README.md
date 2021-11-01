# Benchmarking of various tokenizers

This repository contains benchmarking tools of various tokenizers.

## Overview

To perform benchmarking, you have to run the following two steps.

### Preparation

The following commands prepare resources (e.g. model data) and compile codes.

```
% git submodule update --init
% ./download_resources.sh
% ./compile_all.sh
```

### Measurement

To measure speed of each tokenizer, run the following commands.
If you stop `./run_all.sh` in the middle, `./stats.py` will calculate statistics from avaiable results.

```
% ./run_all.sh | tee ./results
% ./stats.py < ./results
```

## Disclaimer

This software is developed by LegalForce, Inc.,
but not an officially supported LegalForce product.
