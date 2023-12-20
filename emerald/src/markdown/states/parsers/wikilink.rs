use super::ParseResult;
use crate::markdown::utf8_iterator::Utf8Iterator;
use crate::markdown::utils::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub(crate) fn wiki_link(it: &mut Utf8Iterator, start_idx: usize) -> ParseResult {
    if consume_expected_chars!(it, '[').is_none() {
        return ParseResult::Failed;
    }

    if consume_expected_chars!(it, '[').is_none() {
        return ParseResult::Failed;
    }
    loop {
        // end of file detection
        let IterResult::Some((idx, i)) = consume!(it) else {
            break;
        };

        match i {
            ']' => {
                // Match ]] ...
                if consume_expected_chars!(it, ']').is_some() {
                    return ParseResult::Yield(start_idx, idx + 2);
                } else {
                    return ParseResult::Failed;
                }
            }

            '[' => {
                return ParseResult::Failed;
            }
            _ => (),
        }
    }
    ParseResult::Failed
}
