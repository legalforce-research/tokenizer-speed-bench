use std::io::BufRead;

use lindera::dictionary::DictionaryConfig;
use lindera::mode::Mode;
use lindera::tokenizer::{Tokenizer, TokenizerConfig};
use lindera::DictionaryKind;

fn main() {
    let dictionary = DictionaryConfig {
        kind: Some(DictionaryKind::UniDic),
        path: None,
    };

    let config = TokenizerConfig {
        dictionary,
        user_dictionary: None,
        mode: Mode::Normal,
    };

    let tokenizer = Tokenizer::from_config(config).unwrap();
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
    println!("Elapsed-lindera-unidic: {} [sec]", duration.as_secs_f64());

    dbg!(n_words);
}
