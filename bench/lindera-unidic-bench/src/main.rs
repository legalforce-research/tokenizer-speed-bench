use std::io::BufRead;

use lindera::tokenizer::Tokenizer;

fn main() {
    let tokenizer = Tokenizer::new().unwrap();

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
