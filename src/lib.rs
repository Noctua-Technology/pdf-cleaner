mod utils;

use lopdf::Document;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, pdf-parser!");
}

#[wasm_bindgen]
pub fn replace_text() {
    let mut doc = Document::load("assets/example.pdf").unwrap();

    doc.version = "1.4".to_string();

    doc.replace_text(1, "Hello World!", "Modified text!", None)
        .unwrap();

    if false {
        doc.save("modified.pdf").unwrap();
    }
}
