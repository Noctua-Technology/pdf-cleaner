use std::fs;

use lopdf::content::Content;

use pdf_cleaner::{self, text::PDF_TEXT_OPERATORS};

#[test]
fn removes_all_text_operations() {
    // Read the sample PDF from the repo's mock directory
    let input = fs::read("tests/files/test.pdf").expect("failed to read mock/test.pdf");

    // Call the public wrapper exposed in `lib.rs`. It returns Result<Vec<u8>, JsValue>
    let cleaned = pdf_cleaner::remove_text(input).expect("remove_text returned an error");

    // Load the cleaned PDF and inspect page content streams
    let doc = lopdf::Document::load_mem(&cleaned).expect("failed to load cleaned pdf");

    for (page_num, page_id) in doc.get_pages() {
        let content_bytes = doc
            .get_page_content(page_id)
            .expect("failed to get page content");

        let content = Content::decode(&content_bytes).expect("failed to decode content");

        for op in content.operations.iter() {
            let op_name = op.operator.as_str();

            assert!(
                !PDF_TEXT_OPERATORS.contains(&op_name),
                "Found text operator '{}' on page {}",
                op_name,
                page_num
            );
        }
    }
}
