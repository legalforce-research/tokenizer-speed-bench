const vaporetto = require('./pkg/vaporetto_deno_bench.js');

const lines = [];
const reader = require('readline').createInterface({
    input: process.stdin,
});

reader.on('line', (line) => {
    lines.push(line);
});

reader.on('close', () => {
    const tokenizer = vaporetto.Vaporetto.new();
    const t0 = performance.now();
    for (let line of lines) {
        tokenizer.tokenize(line);
    }
    const t1 = performance.now();
    console.log(`Elapsed-vaporetto-wasm: ${(t1 - t0) / 1000} [sec]`);
});
