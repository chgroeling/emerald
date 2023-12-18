use super::markdown_iterator_state::{ActionResult, State, StateData, Yield};
use super::states;
use super::utils::*;
use crate::types;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[derive(Debug)]
pub struct MarkdownAnalyzerIter<'a> {
    buf: &'a str,
    state_data: StateData<'a>,
}

impl<'a> MarkdownAnalyzerIter<'a> {
    pub fn new(buf: &'a str) -> Self {
        Self {
            buf,
            state_data: StateData {
                state: State::DocumentStart,
                it: buf.char_indices().peekable(),
            },
        }
    }

    fn detect_inline_code_block(state_data: &mut StateData, start_idx: usize) -> ActionResult {
        let open_cnt = gather!(state_data.it, Option::<i32>::None, ' ');

        if open_cnt < 4 {
            return ActionResult::NextState(State::EmptyLine);
        }

        // Opening Inline Code Block was detected
        let mut act_idx = start_idx + open_cnt as usize - 1; // -1 because the first char doesn't count

        loop {
            // end of file detection
            let ConsumeResult::Some((idx, i)) = consume!(state_data.it) else {
                break;
            };

            if i == '\n' {
                return ActionResult::YieldState(
                    State::InlCodeBlock,
                    Yield::CodeBlock(start_idx, idx),
                );
            }

            act_idx = idx;
        }

        // end of file handling
        ActionResult::YieldState(
            State::InlCodeBlock,
            Yield::CodeBlock(start_idx, act_idx + 1),
        )
    }

    fn inline_codeblock_state(state_data: &mut StateData) -> ActionResult {
        let Some((index, i)) = state_data.it.peek().cloned() else {
            return ActionResult::EndOfFile;
        };

        match i {
            ' ' => Self::detect_inline_code_block(state_data, index),
            _ => ActionResult::NextState(State::Text),
        }
    }

    fn convert_yield_res_to_md_block(&self, inp: Yield) -> types::MdBlock<'a> {
        match inp {
            Yield::YamlFrontmatter(s, e) => types::MdBlock::YamlFrontmatter(&self.buf[s..e]),
            Yield::CodeBlock(s, e) => types::MdBlock::CodeBlock(&self.buf[s..e]),
            Yield::WikiLink(s, e) => types::MdBlock::WikiLink(&self.buf[s..e]),
            Yield::Link(s, e) => types::MdBlock::Link(&self.buf[s..e]),
        }
    }
}

impl<'a> Iterator for MarkdownAnalyzerIter<'a> {
    type Item = types::MdBlock<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let ar: ActionResult = match self.state_data.state {
                State::DocumentStart => states::document_start(&mut self.state_data),
                State::EmptyLine => states::empty_line(&mut self.state_data),
                State::NewLine => states::new_line(&mut self.state_data),
                State::YamlFrontmatter => states::yaml_frontmatter(&mut self.state_data),
                State::InlCodeBlock => Self::inline_codeblock_state(&mut self.state_data),
                State::Text => states::text(&mut self.state_data),
            };

            match ar {
                ActionResult::EndOfFile => {
                    return None;
                }
                ActionResult::NextState(state) => {
                    self.state_data.state = state;
                }
                ActionResult::YieldState(state, yield_state) => {
                    self.state_data.state = state;
                    return Some(self.convert_yield_res_to_md_block(yield_state));
                }
            }
        }
    }
}
