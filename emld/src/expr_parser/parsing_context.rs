use std::collections::HashMap;

use super::{output_format::OutputFormat, peek_char_iterator::PeekCharIterator};

pub struct ParsingContext<'a, T> {
    pub key_value: &'a HashMap<&'a str, String>,
    pub iter: PeekCharIterator,
    pub vout: Vec<T>,
    pub format: OutputFormat,
}
