use super::ParseResult;
use crate::markdown::states::markdown_iterator_state::StateData;
use crate::markdown::utils::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub(crate) fn wiki_link(state_data: &mut StateData, start_idx: usize) -> ParseResult {
    // save position of iterator ... needed for backtracking
    let it_pos = state_data.it.get_pos();
    if consume_expected_chars!(state_data.it, '[').is_none() {
        return ParseResult::Failed;
    }

    if consume_expected_chars!(state_data.it, '[').is_none() {
        // backtrack if the link was not a wikilink
        state_data.it.set_pos(it_pos);
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
                    // backtrack if the link was not a wikilink
                    state_data.it.set_pos(it_pos);
                    return ParseResult::Failed;
                }
            }

            '[' => {
                // backtrack if the link was not a wikilink
                state_data.it.set_pos(it_pos);
                return ParseResult::Failed;
            }
            _ => (),
        }
    }
    // backtrack if the link was not a wikilink
    state_data.it.set_pos(it_pos);
    ParseResult::Failed
}
