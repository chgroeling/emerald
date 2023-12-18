use super::markdown_iterator_state::{ActionResult, State, StateData};
use super::parsers;
use crate::markdown::utils::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub(crate) fn text(state_data: &mut StateData) -> ActionResult {
    let Some((index, i)) = state_data.it.peek().cloned() else {
        return ActionResult::EndOfFile;
    };

    match i {
        // # Text
        '[' => parsers::link_or_wikilink(state_data, index),
        '`' => parsers::code_block(state_data, index),
        '\n' => {
            consume!(state_data.it);
            ActionResult::NextState(State::NewLine)
        }

        _ => {
            consume!(state_data.it);
            ActionResult::NextState(State::Text)
        }
    }
}
