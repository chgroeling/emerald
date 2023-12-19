use super::ParseResult;
use crate::markdown::states::markdown_iterator_state::StateData;
use crate::markdown::utils::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub(crate) fn link(state_data: &mut StateData, start_idx: usize) -> ParseResult {
    if consume_expected_chars!(state_data.it, '[').is_none() {
        return ParseResult::Failed;
    }

    enum LinkState {
        LinkStart(usize),
        LinkDescriptionFinished(usize),
    }

    // Opening of an internal link was detected
    let mut link_state: LinkState = LinkState::LinkStart(start_idx);

    loop {
        // end of file detection
        let IterResult::Some((idx, i)) = consume!(state_data.it) else {
            break;
        };

        link_state = match link_state {
            LinkState::LinkStart(start_idx) if i == ']' => {
                // next char must be '('
                if consume_expected_chars!(state_data.it, '(').is_some() {
                    LinkState::LinkDescriptionFinished(start_idx)
                } else {
                    return ParseResult::Failed;
                }
            }
            LinkState::LinkDescriptionFinished(start_idx) if i == ')' => {
                return ParseResult::Yield(start_idx, idx + 1);
            }

            _ => link_state,
        };
    }

    ParseResult::Failed
}
