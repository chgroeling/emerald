use super::markdown_iterator_state::{ActionResult, State, StateData, Yield};
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
        '[' => {
            let it_pos = state_data.it.get_pos();
            let res = parsers::detect_wiki_link(state_data, index);
            let out = match res {
                parsers::ParseResult::Failed => {
                    state_data.it.set_pos(it_pos);
                    let res2 = parsers::detect_link(state_data, index);

                    match res2 {
                        parsers::ParseResult::Failed => ActionResult::NextState(State::NewLine),
                        parsers::ParseResult::Yield(s, e) => {
                            ActionResult::YieldState(State::Text, Yield::Link(s, e))
                        }
                    }
                }
                parsers::ParseResult::Yield(s, e) => {
                    ActionResult::YieldState(State::Text, Yield::WikiLink(s, e))
                }
            };

            out
        }
        '`' => match parsers::code_block(state_data, index) {
            parsers::ParseResult::Failed => ActionResult::NextState(State::NewLine),
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
