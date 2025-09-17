mod clean_pdf;
mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = filterOperations)]
pub fn filter_operations(bytes: Vec<u8>, operators: Vec<String>) -> Result<Vec<u8>, JsValue> {
    match clean_pdf::filter_operations(bytes, operators.iter().map(AsRef::as_ref).collect()) {
        Ok(cleaned_bytes) => Ok(cleaned_bytes),
        Err(e) => Err(JsValue::from_str(&format!("Error: {}", e))),
    }
}
