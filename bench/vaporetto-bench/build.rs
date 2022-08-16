//! Pre-compiler for a predictor from a given model.

use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Cursor};
use std::path::PathBuf;

use vaporetto::{KyteaModel, Model};

fn main() {
    let model_data = Cursor::new(include_bytes!("../../resources/jp-0.4.7-5.mod"));
    let model = KyteaModel::read(model_data).unwrap();
    let model = Model::try_from(model).unwrap();
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut buf = BufWriter::new(File::create(out.join("predictor.bin")).unwrap());
    model.write(&mut buf).unwrap();
}
