use std::io::Cursor;
use std::convert::TryFrom;
use std::str::FromStr;

use js_sys::{Array, Object, JsString};
use vaporetto::{Model, Predictor, Sentence, KyteaModel};
use wasm_bindgen::{prelude::*, JsValue};

#[wasm_bindgen]
pub struct Vaporetto {
    predictor: Predictor,
}

#[wasm_bindgen]
impl Vaporetto {
    #[wasm_bindgen]
    pub fn new() -> Self {
        let mut f = Cursor::new(include_bytes!("../../../resources/jp-0.4.7-6.mod"));
        let kytea_model = KyteaModel::read(&mut f).unwrap();
        let model = Model::try_from(kytea_model).unwrap();
        let predictor = Predictor::new(model);
        Self {
            predictor,
        }
    }

    #[wasm_bindgen]
    pub fn tokenize(&self, text: &str) -> Object {
        let s = if let Ok(s) = Sentence::from_raw(text) {
            s
        } else {
            return JsValue::NULL.into();
        };
        let s = self.predictor.predict(s);

        let result = Array::new();
        for word in s.to_tokenized_vec().unwrap() {
            result.push(&JsString::from_str(word).unwrap());
        }
        result.into()
    }
}

impl Default for Vaporetto {
    fn default() -> Self {
        Self::new()
    }
}
