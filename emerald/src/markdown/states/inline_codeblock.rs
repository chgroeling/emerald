use super::parsers;
use crate::markdown::markdown_iterator_state::{ActionResult, State, StateData};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub(crate) fn inline_codeblock(state_data: &mut StateData) -> ActionResult {
    let Some((index, i)) = state_data.it.peek().cloned() else {
        return ActionResult::EndOfFile;
    };

    match i {
        ' ' => parsers::inline_code_block(state_data, index),
        _ => ActionResult::NextState(State::Text),
    }
}
