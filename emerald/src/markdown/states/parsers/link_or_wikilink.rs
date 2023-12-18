use crate::markdown::markdown_iterator_state::{ActionResult, State, StateData, Yield};
use crate::markdown::utils::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

fn detect_wiki_link(state_data: &mut StateData, start_idx: usize) -> ActionResult {
    loop {
        // end of file detection
        let ConsumeResult::Some((idx, i)) = consume!(state_data.it) else {
            break;
        };

        match i {
            ']' => {
                // Match ]] ...
                if consume_expected_chars!(state_data.it, ']').is_some() {
                    return ActionResult::YieldState(
                        State::Text,
                        Yield::WikiLink(start_idx, idx + 2),
                    );
                } else {
                    return ActionResult::NextState(State::Text);
                }
            }

            '[' => {
                return ActionResult::NextState(State::Text);
            }
            _ => (),
        }
    }
    ActionResult::NextState(State::Text)
}

fn detect_link(state_data: &mut StateData, start_idx: usize) -> ActionResult {
    enum LinkState {
        LinkStart(usize),
        LinkDescriptionFinished(usize),
    }

    // Opening of an internal link was detected
    let mut link_state: LinkState = LinkState::LinkStart(start_idx);

    loop {
        // end of file detection
        let ConsumeResult::Some((idx, i)) = consume!(state_data.it) else {
            break;
        };

        link_state = match link_state {
            LinkState::LinkStart(start_idx) if i == ']' => {
                // next char must be '('
                if consume_expected_chars!(state_data.it, '(').is_some() {
                    LinkState::LinkDescriptionFinished(start_idx)
                } else {
                    return ActionResult::NextState(State::Text);
                }
            }
            LinkState::LinkDescriptionFinished(start_idx) if i == ')' => {
                return ActionResult::YieldState(State::Text, Yield::Link(start_idx, idx + 1));
            }

            _ => link_state,
        };
    }

    ActionResult::NextState(State::Text)
}

pub(crate) fn link_or_wikilink(state_data: &mut StateData, start_idx: usize) -> ActionResult {
    if consume_expected_chars!(state_data.it, '[').is_none() {
        return ActionResult::NextState(State::Text);
    }

    if consume_expected_chars!(state_data.it, '[').is_some() {
        // wiki link starts with '[['
        detect_wiki_link(state_data, start_idx)
    } else {
        // conventional link starts with '['
        detect_link(state_data, start_idx)
    }
}
