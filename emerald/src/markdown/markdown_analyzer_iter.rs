use super::states;
use super::utf8_iterator::Utf8Iterator;
use crate::markdown::states::markdown_iterator_state::{ActionResult, State, StateData, Yield};
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
                it: Utf8Iterator::new(buf),
            },
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
                State::InlCodeBlock => states::inline_codeblock(&mut self.state_data),
                State::Text => states::text(&mut self.state_data),
            };

            match ar {
                ActionResult::EndOfFile => {
                    return None;
                }
                ActionResult::Error(state) => {
                    self.state_data.it.next();
                    self.state_data.state = state;
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
