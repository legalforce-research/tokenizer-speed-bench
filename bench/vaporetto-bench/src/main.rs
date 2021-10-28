use std::io::{BufRead, Cursor};
use std::convert::TryFrom;

use vaporetto::{KyteaModel,Model, Sentence, Predictor};

fn main() {
    let mut model_data = Cursor::new(include_bytes!("../../../resources/jp-0.4.7-6.mod"));
    let model = KyteaModel::read(&mut model_data).unwrap();
    let model = Model::try_from(model).unwrap();
    let predictor = Predictor::new(model);
    let mut lines = vec![];
    for line in std::io::stdin().lock().lines() {
        lines.push(line.unwrap());
    }
    let start = std::time::Instant::now();
    let mut n_words = 0;
    for line in lines {
        let s = Sentence::from_raw(line).unwrap();
        let s = predictor.predict(s);
        let toks = s.to_tokenized_vec().unwrap();
        n_words += toks.len();
    }
    let duration = start.elapsed();
    println!("Elapsed-vaporetto: {} [sec]", duration.as_secs_f64());
    dbg!(n_words);
}
