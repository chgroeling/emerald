use super::ParseResult;
use crate::markdown::utf8_iterator::Utf8Iterator;
use crate::markdown::utils::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

/// Detects an empty line in the markdown input.
///
/// This method checks if the current position of the iterator corresponds to
/// an empty line in the markdown input. An empty line is defined as a sequence of
/// spaces followed by an optional carriage return ('\r') and a newline character (`\n`).
/// It consumes all the spaces leading up to the newline character.
///
/// The method is called when parsing markdown to correctly identify and handle
/// empty lines, which can be significant in markdown syntax, especially in
/// determining the boundaries of different markdown elements.
///
/// # Returns
/// * `MarkdownIteratorState::EmptyLineFound` if an empty line is detected.
/// * `MarkdownIteratorState::IllegalFormat` if the next character is not a newline
///   or the end of the iterator is reached, which implies an illegal or unexpected format.
pub(crate) fn empty_line(it: &mut Utf8Iterator, start_idx: usize) -> ParseResult {
    // gather all whitespaces doesnt matter how many
    gather!(it, Option::<i32>::None, ' ');

    // consume optional carriage return
    consume_expected_chars!(it, '\r');

    // check if the following char is a newline
    if let IterResult::Some((index, _)) = consume_expected_chars!(it, '\n') {
        ParseResult::Yield(start_idx, index)
    } else {
        ParseResult::Failed
    }
}
