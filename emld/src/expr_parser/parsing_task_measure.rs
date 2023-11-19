use std::{cmp::max, collections::HashMap};

use super::{
    output_format::OutputFormat, parsing_context::ParsingContext, parsing_task::ParsingTask,
    peek_char_iterator::PeekCharIterator,
};

pub struct ParsingTaskMeasure;
impl ParsingTask for ParsingTaskMeasure {
    type Item = usize;
    type Output = Vec<usize>;

    /// Called in case the context should be initialized
    fn init<'a>(
        inp: &'a str,
        key_value: &'a HashMap<&'a str, String>,
    ) -> ParsingContext<'a, Self::Item> {
        let vec: Vec<_> = inp.chars().collect();
        let mut vout = Vec::<usize>::new();
        vout.push(0);
        ParsingContext::<'_, Self::Item> {
            key_value,
            iter: PeekCharIterator::new(vec),
            vout: vout,
            format: OutputFormat::None,
        }
    }

    fn error(context: &mut ParsingContext<'_, Self::Item>) {
        context.vout[0] += context.iter.get_mark2cur().unwrap().len();
    }

    fn process_char(context: &mut ParsingContext<'_, Self::Item>, _ch: char) {
        context.vout[0] += 1;
    }

    fn process_char_placeholder(_context: &mut ParsingContext<'_, Self::Item>, _ch: char) {}

    fn process_str_placeholder(context: &mut ParsingContext<'_, Self::Item>, arg: String) {
        let Some(repl_str) = context.key_value.get(arg.as_str()) else {
            Self::error(context);
            return;
        };
        let repl_c = repl_str.chars().count();

        match context.format {
            OutputFormat::None => {
                context.vout[0] += repl_c;
                context.vout.push(repl_c);
            }
            OutputFormat::LeftAlign(la) => {
                let repl_c_max = max(repl_c, la as usize);
                context.vout[0] += repl_c_max;
                context.vout.push(repl_c_max);
            }
            OutputFormat::LeftAlignTrunc(la) => {
                let repl_c = la as usize;
                context.vout[0] += repl_c;
                context.vout.push(repl_c);
            }
        }
    }

    fn done(context: ParsingContext<'_, Self::Item>) -> Self::Output {
        context.vout
    }
}
