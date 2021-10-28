use std::io::BufRead;

use sudachi::analysis::stateless_tokenizer::StatelessTokenizer;
use sudachi::config::Config;
use sudachi::dic::dictionary::JapaneseDictionary;
use sudachi::prelude::*;

fn main() {
    let config = Config::new(None, None, None).unwrap();

    let dict = JapaneseDictionary::from_cfg(&config).unwrap();
    let tokenizer = StatelessTokenizer::new(&dict);

    let lines: Vec<_> = std::io::stdin().lock().lines().map(|line| line.unwrap()).collect();
    let mut n_words = 0;

    let start = std::time::Instant::now();

    for line in &lines {
        let tokens = tokenizer.tokenize(line, Mode::C, false).unwrap();
        n_words += tokens.len();
    }

    let duration = start.elapsed();
    println!("Elapsed-sudachi.rs: {} [sec]", duration.as_secs_f64());

    dbg!(n_words);
}
