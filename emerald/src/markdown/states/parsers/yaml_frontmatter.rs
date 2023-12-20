use super::ParseResult;
use crate::markdown::utf8_iterator::Utf8Iterator;
use crate::markdown::utils::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

/// Detects the presence of YAML front matter in markdown input.
///
/// YAML front matter is typically used at the beginning of markdown documents to contain metadata,
/// enclosed within triple-dashed lines (`---`). This method identifies the start and end of the
/// YAML front matter block, if present, and returns a corresponding `MarkdownIteratorState`.
///
/// The detection starts from the current position of the iterator and proceeds until it either confirms
/// the presence of a valid YAML front matter block or encounters a format that invalidates the block.
///
/// # Parameters
/// - `start_idx`: The starting index in the markdown text where the detection should begin.
///
/// # Returns
/// - `ParseResult::Yield(start_idx, end_idx)` if a YAML front matter block
///   is detected, where `start_idx` is the starting index and `end_idx` is the index immediately after
///   the end of the block.
/// - `ParseResult::Failed` if the detected block does not conform to the expected YAML
///    front matter format.
pub(crate) fn yaml_frontmatter(it: &mut Utf8Iterator, start_idx: usize) -> ParseResult {
    // gather 3 dashes
    if gather!(it, Option::<i32>::None, '-') != 3 {
        return ParseResult::Failed;
    }
    // consume optional carriage return
    consume_expected_chars!(it, '\r');

    if consume_expected_chars!(it, '\n').is_none_or_eof() {
        return ParseResult::Failed;
    }

    let mut last_index: usize = 0;
    loop {
        let IterResult::Some((index, i)) = consume!(it) else {
            break;
        };

        last_index = index; // needed in case of eof
        if i == '\n' {
            // assume a dash after a newline
            if consume_expected_chars!(it, '-').is_none_or_eof() {
                continue;
            }

            last_index += 1;

            // gather 2 more dashes
            let dash_cnt = gather!(it, Option::<i32>::None, '-');
            if dash_cnt != 2 {
                last_index += dash_cnt as usize;
                continue;
            }
            // +1 since the index relates to the newline
            // +3 since 3 dashes were found
            let mut end_index = 1 + index + 3;

            // gather all whitespaces doesnt matter how many
            let ws_count = gather!(it, Option::<i32>::None, ' ');
            end_index += ws_count as usize;

            // consume optional carriage return
            if consume_expected_chars!(it, '\r').is_some() {
                end_index += 1;
            }

            if consume_expected_chars!(it, '\n').is_some() {
                end_index += 1usize;
                return ParseResult::Yield(start_idx, end_index);
            }
        }
    }

    ParseResult::Yield(start_idx, last_index + 1)
}
