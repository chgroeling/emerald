use super::parsers;
use super::state::{ActionResult, State, StateData, Yield};
use crate::markdown::utils::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub(crate) fn text(state_data: &mut StateData) -> ActionResult {
    let Some((index, i)) = state_data.it.peek().cloned() else {
        return ActionResult::EndOfFile;
    };

    match i {
        // # Text
        '[' => {
            // save position of iterator ... needed for backtracking
            let it_pos = state_data.it.get_pos();
            match parsers::wiki_link(&mut state_data.it, index) {
                parsers::ParseResult::Failed => {
                    // backtrack if the link was not a wikilink
                    state_data.it.set_pos(it_pos);
                }
                parsers::ParseResult::Yield(s, e) => {
                    return ActionResult::YieldState(State::Text, Yield::WikiLink(s, e))
                }
            };
            match parsers::link(&mut state_data.it, index) {
                parsers::ParseResult::Failed => {
                    // backtrack if the link was not a wikilink
                    state_data.it.set_pos(it_pos);
                    ActionResult::Error(State::NewLine)
                }
                parsers::ParseResult::Yield(s, e) => {
                    ActionResult::YieldState(State::Text, Yield::Link(s, e))
                }
            }
        }
        '`' => match parsers::code_block(&mut state_data.it, index) {
            parsers::ParseResult::Failed => ActionResult::Error(State::NewLine),
            parsers::ParseResult::Yield(s, e) => {
                ActionResult::YieldState(State::Text, Yield::CodeBlock(s, e))
            }
        },
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
