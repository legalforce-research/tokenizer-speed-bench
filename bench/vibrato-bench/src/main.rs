use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use vibrato::{Dictionary, Tokenizer};

fn main() {
    let args: Vec<String> = env::args().collect();

    let root_dir = env!("CARGO_MANIFEST_DIR");
    let dictname = &args[1];

    let reader = BufReader::new(
        File::open(format!("{}/resources_{}/system.dic", root_dir, dictname)).unwrap(),
    );
    let dict = unsafe { Dictionary::read_unchecked(reader).unwrap() };
    let mut tokenizer = Tokenizer::new(&dict);

    let lines: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let mut n_words = 0;

    let start = std::time::Instant::now();
    for line in &lines {
        let tokens = tokenizer.tokenize(line).unwrap();
        n_words += tokens.len();
    }
    let duration = start.elapsed();

    println!(
        "Elapsed-vibrato-{}: {} [sec]",
        dictname,
        duration.as_secs_f64()
    );

    dbg!(n_words);
}