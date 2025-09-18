pub mod filter_operations;
pub mod remove_text;
mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = filterOperations)]
pub fn filter_operations(bytes: Vec<u8>, operators: Vec<String>) -> Result<Vec<u8>, JsValue> {
    match filter_operations::filter_operations(bytes, operators.iter().map(AsRef::as_ref).collect())
    {
        Ok(cleaned_bytes) => Ok(cleaned_bytes),
        Err(e) => Err(JsValue::from_str(&format!("Error: {}", e))),
    }
}

#[wasm_bindgen(js_name = removeText)]
pub fn remove_text(bytes: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    match remove_text::remove_text(bytes) {
        Ok(cleaned_bytes) => Ok(cleaned_bytes),
        Err(e) => Err(JsValue::from_str(&format!("Error: {}", e))),
    }
}
