#!/usr/bin/env python3

import importlib
import os
import sys
import time


if __name__ == "__main__":
    sys.path.append(os.path.join(os.path.dirname(__file__), "../../thirdparty/janome"))
    tokenizer_mod = importlib.import_module("janome.tokenizer")

    tokenizer = tokenizer_mod.Tokenizer()

    lines = [line.rstrip("\n") for line in sys.stdin]

    start = time.time()
    n_tokens = 0
    for line in lines:
        n_tokens += sum(1 for _ in tokenizer.tokenize(line))
    end = time.time()

    print(f"Elapsed-janome: {end - start} [sec]")

    print(n_tokens)
