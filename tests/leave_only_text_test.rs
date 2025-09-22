use std::fs;

use lopdf::content::Content;

use pdf_cleaner::{self, text::PDF_TEXT_OPERATORS};

#[test]
fn keeps_only_text_operations() {
    // Read the sample PDF from the repo's mock directory
    let input = fs::read("tests/files/test.pdf").expect("failed to read mock/test.pdf");

    // Load the original PDF to inspect baseline operator counts
    let original_doc = lopdf::Document::load_mem(&input).expect("failed to load original pdf");

    // Counts across the whole document
    let mut original_text_ops = 0usize;
    let mut original_nontext_ops = 0usize;

    for (_page_num, page_id) in original_doc.get_pages() {
        let content_bytes = original_doc
            .get_page_content(page_id)
            .expect("failed to get page content from original");

        let content = Content::decode(&content_bytes).expect("failed to decode original content");

        for op in content.operations.iter() {
            let op_name = op.operator.as_str();
            if PDF_TEXT_OPERATORS.contains(&op_name) {
                original_text_ops += 1;
            } else {
                original_nontext_ops += 1;
            }
        }
    }

    // Sanity: original must have at least some non-text operators to remove
    assert!(
        original_nontext_ops > 0,
        "Test input has no non-text operators to remove"
    );
    assert!(original_text_ops > 0, "Test input has no text operators");

    // Call the public wrapper exposed in `lib.rs`. It returns Result<Vec<u8>, JsValue>
    let cleaned = pdf_cleaner::leave_only_text(input).expect("leave_only_text returned an error");

    // Load the cleaned PDF and inspect page content streams
    let doc = lopdf::Document::load_mem(&cleaned).expect("failed to load cleaned pdf");

    let mut cleaned_text_ops = 0usize;
    let mut cleaned_nontext_ops = 0usize;

    for (page_num, page_id) in doc.get_pages() {
        let content_bytes = doc
            .get_page_content(page_id)
            .expect("failed to get page content");

        let content = Content::decode(&content_bytes).expect("failed to decode content");

        // Count ops
        for op in content.operations.iter() {
            let op_name = op.operator.as_str();
            if PDF_TEXT_OPERATORS.contains(&op_name) {
                cleaned_text_ops += 1;
            } else {
                cleaned_nontext_ops += 1;
            }
        }

        // Ensure there are operations left
        assert!(
            !content.operations.is_empty(),
            "No operations left on page {}",
            page_num
        );
    }

    // Stronger assertions:
    // - No non-text operators remain anywhere
    assert_eq!(
        cleaned_nontext_ops, 0,
        "Expected all non-text operators to be removed, but found {}",
        cleaned_nontext_ops
    );

    // - All original text operators are still present (count should be >= original_text_ops)
    assert!(
        cleaned_text_ops >= original_text_ops,
        "Expected cleaned text op count ({}) to be >= original text op count ({})",
        cleaned_text_ops,
        original_text_ops
    );

    // - Total operator count equals cleaned_text_ops and is less than or equal to original total
    let original_total = original_text_ops + original_nontext_ops;
    let cleaned_total = cleaned_text_ops + cleaned_nontext_ops;

    assert_eq!(cleaned_total, cleaned_text_ops);

    assert!(
        cleaned_total <= original_total,
        "Cleaned document has more operations than original"
    );
}
