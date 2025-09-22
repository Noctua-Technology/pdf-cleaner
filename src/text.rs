use std::error::Error;

use crate::filter_operations::{filter_operations, Mode};

pub const PDF_TEXT_OPERATORS: [&str; 17] = [
    "BT", // Begin text object
    "ET", // End text object
    "Tj", // Show text
    "TJ", // Show text (with positioning)
    "'",  // Move to next line and show text
    "\"", // Set spacing, move to next line, and show text
    "Tf", // Set text font and size
    "Tc", // Set character spacing
    "Tw", // Set word spacing
    "Tz", // Set horizontal scaling
    "TL", // Set text leading
    "Tr", // Set text rendering mode
    "Ts", // Set text rise
    "Td", // Move text position
    "TD", // Move text position and set leading
    "Tm", // Set text matrix
    "T*", // Move to start of next line
];

/**
 * Parses an existing PDF, removes all text operations,
 * and saves the result to a new file.
 */
pub fn remove_text(buffer: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    filter_operations(buffer, PDF_TEXT_OPERATORS.to_vec(), Mode::Remove)
}

pub fn leave_only_text(buffer: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    filter_operations(buffer, PDF_TEXT_OPERATORS.to_vec(), Mode::Keep)
}
