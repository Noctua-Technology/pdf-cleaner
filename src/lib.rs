pub mod filter_operations;
pub mod text;
mod utils;

use wasm_bindgen::prelude::*;

use crate::filter_operations::Mode;

#[wasm_bindgen(js_name = filterOperations)]
pub fn filter_operations(
    bytes: Vec<u8>,
    operators: Vec<String>,
    mode: Mode,
) -> Result<Vec<u8>, JsValue> {
    match filter_operations::filter_operations(
        bytes,
        operators.iter().map(AsRef::as_ref).collect(),
        mode,
    ) {
        Ok(cleaned_bytes) => Ok(cleaned_bytes),
        Err(e) => Err(JsValue::from_str(&format!("Error: {}", e))),
    }
}

#[wasm_bindgen(js_name = removeText)]
pub fn remove_text(bytes: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    match text::remove_text(bytes) {
        Ok(cleaned_bytes) => Ok(cleaned_bytes),
        Err(e) => Err(JsValue::from_str(&format!("Error: {}", e))),
    }
}

#[wasm_bindgen(js_name = leaveOnlyText)]
pub fn leave_only_text(bytes: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    match text::leave_only_text(bytes) {
        Ok(cleaned_bytes) => Ok(cleaned_bytes),
        Err(e) => Err(JsValue::from_str(&format!("Error: {}", e))),
    }
}
