use super::markdown_iterator_state::{ActionResult, State, StateData};
use super::parsers;
use crate::markdown::utils::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub(crate) fn new_line(state_data: &mut StateData) -> ActionResult {
    let Some((_, i)) = state_data.it.peek().cloned() else {
        return ActionResult::EndOfFile;
    };

    match i {
        // # New line found
        ' ' => match parsers::empty_line(state_data) {
            parsers::ParseResult::Failed => ActionResult::NextState(State::Text),
            parsers::ParseResult::Ok => ActionResult::NextState(State::EmptyLine),
            parsers::ParseResult::Yield(_, _) => panic!("Doesn't yield"),
        },
        '\n' => {
            consume!(state_data.it);
            ActionResult::NextState(State::EmptyLine)
        }
        _ => ActionResult::NextState(State::Text),
    }
}
