use lopdf::{
    content::{Content, Operation},
    Dictionary, Document, Object, Stream,
};
use std::collections::HashSet;
use std::error::Error;

/**
 * Parses an existing PDF, removes all specified operations.
 */
pub fn filter_operations(buffer: Vec<u8>, operators: Vec<&str>) -> Result<Vec<u8>, Box<dyn Error>> {
    // 1. Load the document
    let mut doc = Document::load_mem(buffer.as_slice())?;

    // 2. Define the set of all PDF text operators
    let text_operators: HashSet<&str> = operators.iter().cloned().collect();

    // 3. Iterate over all pages
    for (page_num, page_id) in doc.get_pages() {
        // Get the page's content stream(s)
        let content_bytes = doc.get_page_content(page_id)?;

        // Decode the bytes into a list of operations
        let content = Content::decode(&content_bytes)?;

        // 4. Filter the operations
        let non_text_operations: Vec<Operation> = content
            .operations
            .into_iter()
            .filter(|op| !text_operators.contains(op.operator.as_str()))
            .collect();

        // 5. Create a new content stream
        let new_content = Content {
            operations: non_text_operations,
        };

        let new_content_bytes = new_content.encode()?;
        let new_stream = Stream::new(Dictionary::new(), new_content_bytes);
        let new_stream_id = doc.add_object(Object::Stream(new_stream));

        // 6. Update the page to point to the new stream
        let page_dict = doc.get_object_mut(page_id)?.as_dict_mut().map_err(|e| {
            format!(
                "Page object {} (ID: {:?}) is not a dictionary: {}",
                page_num, page_id, e
            )
        })?;

        page_dict.set("Contents", Object::Reference(new_stream_id));
    }

    let mut output = Vec::new();

    doc.save_to(&mut output)?;

    Ok(output)
}
