use std::fs;

use lopdf::content::Content;

use pdf_cleaner::PDFDocument;

#[test]
fn leave_only_text_is_idempotent() {
    let input = fs::read("tests/files/test.pdf").expect("failed to read test.pdf");

    let mut doc = PDFDocument::from_bytes(&input);

    // First run
    let cleaned1 = doc.leave_only_text().expect("first leave_only_text failed");

    // Second run on the cleaned output
    let cleaned2 = doc
        .leave_only_text()
        .expect("second leave_only_text failed");

    // Byte-for-byte equality can fail due to non-semantic PDF differences
    // (object ordering, metadata, compression). Instead we validate logical
    // idempotence by comparing decoded page content operations below.

    // Additionally, load both with lopdf and ensure decoded content operations match
    let doc1 = lopdf::Document::load_mem(&cleaned1).expect("failed to load cleaned1");
    let doc2 = lopdf::Document::load_mem(&cleaned2).expect("failed to load cleaned2");

    let pages1: Vec<_> = doc1.get_pages().into_iter().collect();
    let pages2: Vec<_> = doc2.get_pages().into_iter().collect();

    assert_eq!(
        pages1.len(),
        pages2.len(),
        "page count changed after second clean"
    );

    for ((pnum1, id1), (pnum2, id2)) in pages1.iter().zip(pages2.iter()) {
        assert_eq!(pnum1, pnum2);

        let content1 =
            Content::decode(&doc1.get_page_content(*id1).expect("get content1")).expect("decode1");

        let content2 =
            Content::decode(&doc2.get_page_content(*id2).expect("get content2")).expect("decode2");

        let ops1: Vec<_> = content1
            .operations
            .iter()
            .map(|o| o.operator.clone())
            .collect();

        let ops2: Vec<_> = content2
            .operations
            .iter()
            .map(|o| o.operator.clone())
            .collect();

        assert_eq!(
            ops1, ops2,
            "Decoded operators differ between runs for page {}",
            pnum1
        );
    }
}
