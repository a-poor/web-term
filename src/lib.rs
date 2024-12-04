mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, web-term!");
}

#[wasm_bindgen]
pub fn split_term_text(text: &str) -> Result<Vec<String>, JsValue> {
    match shellwords::split(text) {
        Ok(v) => Ok(v),
        Err(e) => Err(JsValue::from_str(&format!("{}", e))),
    }
}

