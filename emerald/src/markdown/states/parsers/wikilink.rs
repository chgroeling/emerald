use super::ParseResult;
use crate::markdown::states::markdown_iterator_state::StateData;
use crate::markdown::utils::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub(crate) fn wiki_link(state_data: &mut StateData, start_idx: usize) -> ParseResult {
    if consume_expected_chars!(state_data.it, '[').is_none() {
        return ParseResult::Failed;
    }

    if consume_expected_chars!(state_data.it, '[').is_none() {
        return ParseResult::Failed;
    }
    loop {
        // end of file detection
        let IterResult::Some((idx, i)) = consume!(state_data.it) else {
            break;
        };

        match i {
            ']' => {
                // Match ]] ...
                if consume_expected_chars!(state_data.it, ']').is_some() {
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
