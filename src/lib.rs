mod filter_operations;
mod text;
mod utils;

use crate::filter_operations::Mode;
use lopdf::Document;
pub use text::PDF_TEXT_OPERATORS;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PDFDocument {
    doc: Document,
}

#[wasm_bindgen]
impl PDFDocument {
    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let doc = Document::load_mem(&bytes).expect("Failed to load PDF document");

        PDFDocument { doc }
    }

    #[wasm_bindgen(js_name = filterOperations)]
    pub fn filter_operations(
        &mut self,
        operators: Vec<String>,
        mode: Mode,
    ) -> Result<Vec<u8>, JsValue> {
        match filter_operations::filter_operations(
            &mut self.doc,
            operators.iter().map(AsRef::as_ref).collect(),
            mode,
        ) {
            Ok(cleaned_bytes) => Ok(cleaned_bytes),
            Err(e) => Err(JsValue::from_str(&format!("Error: {}", e))),
        }
    }

    #[wasm_bindgen(js_name = removeText)]
    pub fn remove_text(&mut self) -> Result<Vec<u8>, JsValue> {
        match text::remove_text(&mut self.doc) {
            Ok(cleaned_bytes) => Ok(cleaned_bytes),
            Err(e) => Err(JsValue::from_str(&format!("Error: {}", e))),
        }
    }

    #[wasm_bindgen(js_name = leaveOnlyText)]
    pub fn leave_only_text(&mut self) -> Result<Vec<u8>, JsValue> {
        match text::leave_only_text(&mut self.doc) {
            Ok(cleaned_bytes) => Ok(cleaned_bytes),
            Err(e) => Err(JsValue::from_str(&format!("Error: {}", e))),
        }
    }
}
