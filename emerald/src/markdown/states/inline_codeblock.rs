use super::parsers;
use super::state::{ActionResult, State, StateData, Yield};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub(crate) fn inline_codeblock(state_data: &mut StateData) -> ActionResult {
    let Some((index, i)) = state_data.it.peek().cloned() else {
        return ActionResult::EndOfFile;
    };

    match i {
        ' ' => match parsers::inline_code_block(&mut state_data.it, index) {
            parsers::ParseResult::Failed => ActionResult::NextState(State::EmptyLine),
            parsers::ParseResult::Yield(s, e) => {
                ActionResult::YieldState(State::InlCodeBlock, Yield::CodeBlock(s, e))
            }
        },
        _ => ActionResult::NextState(State::Text),
    }
}
