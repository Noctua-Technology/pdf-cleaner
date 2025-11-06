use lopdf::{
    content::{Content, Operation},
    Dictionary, Document, Object, Stream,
};
use std::collections::HashSet;
use std::error::Error;
use wasm_bindgen::prelude::*;

#[derive(PartialEq)]
#[wasm_bindgen]
pub enum Mode {
    Keep,
    Remove,
}

/**
 * Parses an existing PDF, removes all specified operations.
 */
pub fn filter_operations(
    doc: &mut Document,
    operators: Vec<&str>,
    mode: Mode,
) -> Result<Vec<u8>, Box<dyn Error>> {
    // 2. Define the set of all PDF text operators
    let text_operators: HashSet<&str> = operators.iter().cloned().collect();

    // 3. Iterate over all pages
    for (page_num, page_id) in doc.get_pages() {
        // Get the page's content stream(s)
        let content_bytes = doc.get_page_content(page_id)?;

        // Decode the bytes into a list of operations
        let content = Content::decode(&content_bytes)?;

        // 4. Filter the operations
        let remaining_operations: Vec<Operation> = content
            .operations
            .into_iter()
            .filter(|op| {
                if mode == Mode::Keep {
                    text_operators.contains(op.operator.as_str())
                } else {
                    !text_operators.contains(op.operator.as_str())
                }
            })
            .collect();

        // The two added on remaning_operations is to account for the 'q' and 'cm' operations that we are adding.
        let mut new_streamed_operations: Vec<Operation> =
            Vec::with_capacity(remaining_operations.len() + 2);

        // Save Graphics State (q): Ensure clean state management.
        new_streamed_operations.push(Operation::new("q", vec![]));

        // Add the filtered operations - moves all the remaining content.
        new_streamed_operations.extend(remaining_operations);

        // Restore Graphics State (Q) cleanup after the block.
        new_streamed_operations.push(Operation::new("Q", vec![]));

        // Create a new content stream
        let new_content = Content {
            operations: new_streamed_operations,
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
