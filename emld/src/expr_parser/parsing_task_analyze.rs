use super::output_format::OutputFormat;
use super::parsing_context::ParsingContext;
use super::parsing_task::ParsingTask;
use super::peek_char_iterator::PeekCharIterator;

use std::collections::HashMap;

pub struct ParsingTaskAnalyze;
impl ParsingTask for ParsingTaskAnalyze {
    type Item = String;
    type Output = Vec<String>;

    /// Called in case the context should be initialized
    fn init<'a>(
        inp: &'a str,
        key_value: &'a HashMap<&'a str, String>,
    ) -> ParsingContext<'a, Self::Item> {
        let vec: Vec<_> = inp.chars().collect();
        let vout = Vec::<Self::Item>::new();
        ParsingContext::<'_, Self::Item> {
            key_value,
            iter: PeekCharIterator::new(vec),
            vout: vout,
            format: OutputFormat::None,
        }
    }

    fn error(_context: &mut ParsingContext<'_, Self::Item>) {}

    fn process_char(_context: &mut ParsingContext<'_, Self::Item>, _ch: char) {}

    fn process_char_placeholder(_context: &mut ParsingContext<'_, Self::Item>, _ch: char) {}

    fn process_str_placeholder(_cntext: &mut ParsingContext<'_, Self::Item>, _arg: String) {}

    fn done(context: ParsingContext<'_, Self::Item>) -> Self::Output {
        context.vout
    }
}
