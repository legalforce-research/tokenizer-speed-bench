use std::io::BufRead;

fn main() {
    let mut lines = vec![];
    for line in std::io::stdin().lock().lines() {
        lines.push(line.unwrap());
    }

    let mut n_words = 0;
    let start = std::time::Instant::now();
    for line in lines {
        n_words += tinysegmenter::tokenize(&line).len();
    }
    let duration = start.elapsed();

    println!("Elapsed-rust-tinysegmenter: {} [sec]", duration.as_secs_f64());
    dbg!(n_words);
}
