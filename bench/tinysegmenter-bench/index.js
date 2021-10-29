const TinySegmenter = require('../../thirdparty/tiny_segmenter/tiny_segmenter-0.2.js');

const lines = [];
const reader = require('readline').createInterface({
    input: process.stdin,
});

reader.on('line', (line) => {
    lines.push(line);
});

reader.on('close', () => {
    const segmenter = new TinySegmenter();
    const t0 = performance.now();
    for (let line of lines) {
        segmenter.segment(line);
    }
    const t1 = performance.now();
    console.log(`Elapsed-tiny-segmenter: ${(t1 - t0) / 1000} [sec]`);
});
