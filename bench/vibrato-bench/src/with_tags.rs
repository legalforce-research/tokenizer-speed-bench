use std::fs::File;
use std::io::BufRead;

use clap::Parser;

use vibrato::{Dictionary, Tokenizer};

#[derive(Parser, Debug)]
#[clap(name = "with_tags", about = "A program to benchmark with tags.")]
struct Args {
    #[clap(long, action)]
    dictname: String,
}

fn main() {
    let args = Args::parse();

    let rootdir = env!("CARGO_MANIFEST_DIR");
    let dictname = args.dictname;

    let reader =
        zstd::Decoder::new(File::open(format!("{rootdir}/{dictname}/system.dic.zst")).unwrap())
            .unwrap();
    let dict = unsafe { Dictionary::read_unchecked(reader).unwrap() };
    let tokenizer = Tokenizer::new(dict);
    let mut worker = tokenizer.new_worker();

    let lines: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let mut n_words = 0;
    let mut n_tags = 0;

    let start = std::time::Instant::now();
    for line in &lines {
        worker.reset_sentence(line);
        worker.tokenize();
        n_words += worker.num_tokens();
        for t in worker.token_iter() {
            n_tags += t.feature().len();
        }
    }
    let duration = start.elapsed();

    println!(
        "Elapsed-vibrato-{} (with tags): {} [sec]",
        dictname,
        duration.as_secs_f64()
    );

    dbg!(n_words);
    dbg!(n_tags);
}
