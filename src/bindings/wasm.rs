use crate::ncda;

use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn checksum(input: &str) -> Result<char, JsValue> {
    ncda::checksum(input).map_err(|e| {
        serde_wasm_bindgen::to_value(&e).expect("Failed to serialize error message to json")
    })
}

#[wasm_bindgen]
pub fn check(input: &str) -> Result<bool, JsValue> {
    match ncda::check(input) {
        Ok(_) => Ok(true),
        Err(e) => {
            Err(serde_wasm_bindgen::to_value(&e)
                .expect("Failed to serialize error message to json"))
        }
    }
}
