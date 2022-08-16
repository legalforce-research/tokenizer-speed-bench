use std::io::{BufRead, Cursor};

use vaporetto::{Model, Predictor, Sentence};

fn main() {
    let model_data = Cursor::new(include_bytes!(concat!(env!("OUT_DIR"), "/predictor.bin")));
    let model = Model::read(model_data).unwrap();
    let predictor = Predictor::new(model, false).unwrap();

    let mut lines = vec![];
    for line in std::io::stdin().lock().lines() {
        lines.push(line.unwrap());
    }

    let mut s = Sentence::from_raw(" ").unwrap();
    let mut n_words = 0;
    let start = std::time::Instant::now();
    for line in lines {
        s.update_raw(line).unwrap();
        predictor.predict(&mut s);
        n_words += s.iter_tokens().count();
    }
    let duration = start.elapsed();

    println!("Elapsed-vaporetto: {} [sec]", duration.as_secs_f64());
    dbg!(n_words);
}
