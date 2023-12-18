use super::parsers;
use crate::markdown::markdown_iterator_state::{ActionResult, State, StateData};
use crate::markdown::utils::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub(crate) fn document_start(state_data: &mut StateData) -> ActionResult {
    let Some((index, i)) = state_data.it.peek().cloned() else {
        return ActionResult::EndOfFile;
    };
    match i {
        // # Start of parsing
        '-' => parsers::yaml_frontmatter(state_data, index),
        ' ' => parsers::inline_code_block(state_data, index),
        '\n' => {
            consume!(state_data.it);
            ActionResult::NextState(State::EmptyLine)
        }
        _ => ActionResult::NextState(State::Text),
    }
}
