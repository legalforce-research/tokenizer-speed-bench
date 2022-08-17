#!/usr/bin/env python3

from __future__ import annotations

import collections
import math
import re
import sys


RE_DICT = [
    ('kytea', re.compile(r'Elapsed-kytea: ([0-9\.]+) \[sec\]')),
    ('vaporetto', re.compile(r'Elapsed-vaporetto: ([0-9\.]+) \[sec\]')),
    ('mecab-ipadic', re.compile(r'Elapsed-mecab-ipadic: ([0-9\.]+) \[sec\]')),
    ('mecab-unidic', re.compile(r'Elapsed-mecab-unidic: ([0-9\.]+) \[sec\]')),
    ('kuromoji', re.compile(r'Elapsed-kuromoji: ([0-9\.]+) \[sec\]')),
    ('lindera', re.compile(r'Elapsed-lindera: ([0-9\.]+) \[sec\]')),
    ('sudachi', re.compile(r'Elapsed-sudachi: ([0-9\.]+) \[sec\]')),
    ('sudachi.rs', re.compile(r'Elapsed-sudachi.rs: ([0-9\.]+) \[sec\]')),
    ('rust-tiny-segmenter', re.compile(r'Elapsed-rust-tiny-segmenter: ([0-9\.]+) \[sec\]')),
    ('vibrato-ipadic-mecab-2_7_0', re.compile(r'Elapsed-vibrato-ipadic-mecab-2_7_0: ([0-9\.]+) \[sec\]')),
    ('vibrato-unidic-mecab-2_1_2', re.compile(r'Elapsed-vibrato-unidic-mecab-2_1_2: ([0-9\.]+) \[sec\]')),
    ('vibrato-unidic-cwj-3_1_0', re.compile(r'Elapsed-vibrato-unidic-cwj-3_1_0: ([0-9\.]+) \[sec\]')),
]


def count_chars() -> int:
    n_chars = 0
    with open('./resources/wagahaiwa_nekodearu.txt') as fp:
        for line in fp:
            n_chars += len(line.rstrip('\n'))
    return n_chars


def mean_std(n_chars: int, times: list[float]) -> (float, float):
    speeds = [n_chars / time for time in times]
    mean = sum(speeds) / len(speeds)
    dist = sum((speed - mean) ** 2 for speed in speeds) / len(speeds)
    return mean, math.sqrt(dist)


def _main():
    n_chars = count_chars()
    times = collections.defaultdict(list)
    for line in sys.stdin:
        for name, r in RE_DICT:
            m = r.match(line)
            if m is not None:
                times[name].append(float(m.group(1)))
                break

    for name, _ in RE_DICT:
        mean, std = mean_std(n_chars, times[name])
        print(f'{name} {mean} {std}')


if __name__ == '__main__':
    _main()
