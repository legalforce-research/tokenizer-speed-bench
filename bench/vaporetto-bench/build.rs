//! Pre-compiler for a predictor from a given model.

use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Cursor, Write};
use std::path::PathBuf;

use vaporetto::{KyteaModel, Model, Predictor};

fn main() {
    let model_data = Cursor::new(include_bytes!("../../resources/jp-0.4.7-5.mod"));
    let model = KyteaModel::read(model_data).unwrap();
    let model = Model::try_from(model).unwrap();
    let predictor = Predictor::new(model, false).unwrap();
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut buf = BufWriter::new(File::create(out.join("predictor.bin")).unwrap());
    let predictor_data = predictor.serialize_to_vec().unwrap();
    buf.write_all(&predictor_data).unwrap();
}
