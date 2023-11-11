use std::collections::HashMap;

/// A char iterator with peek, mark, and backtrack functionalities.
///
/// This iterator operates on a `Vec<char>` and uses indices
/// to mark positions and to return to previous states.
struct PeekCharIterator<'a> {
    // The vector of characters to iterate over.
    chars: &'a Vec<char>,
    // The current index in the vector.
    current_index: usize,
    // An optional index for the peeked character.
    peeked_index: Option<usize>,
    // An optional index marking a saved position in the vector.
    marked_index: Option<usize>,
}

impl<'a> PeekCharIterator<'a> {
    /// Creates a new `PeekCharIterator` for a given `Vec<char>`.
    ///
    /// # Arguments
    ///
    /// * `chars` - The `Vec<char>` to iterate over.
    fn new(chars: &'a Vec<char>) -> Self {
        PeekCharIterator {
            chars,
            current_index: 0,
            peeked_index: None,
            marked_index: None,
        }
    }

    /// Peeks at the next character without changing the iterator's state.
    fn peek(&mut self) -> Option<char> {
        if self.peeked_index.is_none() {
            self.peeked_index = Some(self.current_index);
        }

        self.chars.get(self.peeked_index.unwrap()).copied()
    }

    /// Marks the current position in the iterator.
    fn mark(&mut self) {
        self.marked_index = Some(self.current_index);
    }

    /// Resets the iterator to the last marked position.
    fn backtrack(&mut self) {
        if let Some(index) = self.marked_index {
            self.current_index = index;
            self.peeked_index = None;
        }
    }
}

impl<'a> Iterator for PeekCharIterator<'a> {
    type Item = char;

    /// Returns the next character in the iterator.
    ///
    /// If `peek` was previously called, it returns the peeked character and advances the iterator.
    /// Otherwise, it fetches the next character from the vector.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.peeked_index.take() {
            self.current_index = index + 1;
            return self.chars.get(index).copied();
        }

        let result = self.chars.get(self.current_index).copied();
        self.current_index += 1;
        result
    }
}

enum Format {
    None,
    LeftAlign(u32),
}

struct ParserContext<'a> {
    key_value: &'a HashMap<&'a str, String>,
    iter: &'a mut PeekCharIterator<'a>,
    out_str: &'a mut Vec<char>,
    format: Format,
}

pub struct ExprParser;
impl ExprParser {
    pub fn new() -> Self {
        Self
    }
    fn interpret_named_placeholder(&self, context: &mut ParserContext<'_>) {
        let mut literal = Vec::<char>::new();
        loop {
            let Some(ch) = context.iter.next() else {
                return;
            };

            match ch {
                ')' => break,
                _ => literal.push(ch),
            }
        }

        let literal_str: String = literal.into_iter().collect();

        let Some(value) = context.key_value.get(literal_str.as_str()) else {
            return;
        };

        for c in value.chars() {
            context.out_str.push(c)
        }

        if let Format::LeftAlign(la) = context.format {
            let len_diff = (la as i32) - (value.len() as i32);
            if len_diff > 0 {
                for _i in 0..len_diff {
                    context.out_str.push(' ')
                }
            }
        }

        // Reset format for next Placeholder
        context.format = Format::None;
    }

    fn consume_digit_without_0(&self, context: &mut ParserContext<'_>) -> Option<char> {
        loop {
            let Some(ch) = context.iter.peek() else {
                return None;
            };

            match ch {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    context.iter.next(); // consume
                    return Some(ch);
                }
                _ => {
                    return None;
                }
            }
        }
    }

    fn consume_digit(&self, context: &mut ParserContext<'_>) -> Option<char> {
        loop {
            let Some(ch) = context.iter.peek() else {
                return None;
            };

            match ch {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    context.iter.next(); // consume
                    return Some(ch);
                }
                _ => {
                    return None;
                }
            }
        }
    }

    fn consume_expected_char(
        &self,
        context: &mut ParserContext<'_>,
        expected_char: char,
    ) -> Option<char> {
        loop {
            let Some(ch) = context.iter.peek() else {
                return None;
            };

            if ch == expected_char {
                context.iter.next(); // consume
                return Some(ch);
            } else {
                return None;
            }
        }
    }

    fn consume_decimal(&self, context: &mut ParserContext<'_>) -> Option<u32> {
        let mut decimal_vec = Vec::<char>::new();

        let Some(first_digit) = self.consume_digit_without_0(context) else {
            return None;
        };

        decimal_vec.push(first_digit);
        loop {
            let res_digit = self.consume_digit(context);

            let Some(digit) = res_digit else {
                let decimal_str: String = decimal_vec.into_iter().collect();
                let decimal = decimal_str.parse::<u32>().unwrap();
                return Some(decimal);
            };

            decimal_vec.push(digit);
        }
    }

    fn interpret_format_open(&self, context: &mut ParserContext<'_>) {
        if None == self.consume_expected_char(context, '(') {
            return;
        }
        let Some(decimal) = self.consume_decimal(context) else {
            context.out_str.push('(');
            context.out_str.push(context.iter.next().unwrap());
            return;
        };

        if None == self.consume_expected_char(context, ')') {
            return;
        }

        context.format = Format::LeftAlign(decimal);
    }

    fn interpret_placeholder(&self, context: &mut ParserContext<'_>) {
        loop {
            let Some(ch) = context.iter.next() else {
                return;
            };

            match ch {
                '(' => {
                    self.interpret_named_placeholder(context);
                    return;
                }
                '<' => {
                    self.interpret_format_open(context);
                    return;
                }
                'n' => {
                    context.out_str.push('\n');
                    return;
                }
                '%' => {
                    context.out_str.push('%');
                    return;
                }
                _ => {
                    return;
                }
            }
        }
    }

    pub fn parse(&self, key_value: &HashMap<&str, String>, inp: &str) -> String {
        let vec: Vec<_> = inp.chars().collect();
        let mut iter = PeekCharIterator::new(&vec);
        let mut out_str = Vec::<char>::new();
        let mut context = ParserContext {
            key_value: key_value,
            iter: &mut iter,
            out_str: &mut out_str,
            format: Format::None,
        };

        loop {
            let Some(ch) = context.iter.next() else {
                break;
            };
            match ch {
                '%' => {
                    self.interpret_placeholder(&mut context);
                }
                _ => context.out_str.push(ch),
            }
        }
        out_str.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::expr_parser::ExprParser;

    fn test_parse_helper(key_value: &HashMap<&str, String>, inp: &str, expected_output: &str) {
        let parser = ExprParser::new();

        let out_str = parser.parse(&key_value, inp);
        assert_eq!(out_str, expected_output);
    }

    #[test]
    fn test_parse_empty() {
        let key_value = HashMap::<&str, String>::new();
        test_parse_helper(&key_value, "", "");
    }

    #[test]
    fn test_parse_string_without_tokens() {
        let key_value = HashMap::<&str, String>::new();
        test_parse_helper(&key_value, "Conventional string", "Conventional string");
    }

    #[test]
    fn test_parse_unicode_string_without_tokens() {
        let key_value = HashMap::<&str, String>::new();
        test_parse_helper(&key_value, "Smiley 😊 Smiley", "Smiley 😊 Smiley");
    }

    #[test]
    fn test_parse_string_with_var1_token() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "world".into());
        test_parse_helper(&key_value, "Hello %(var1)", "Hello world");
    }

    #[test]
    fn test_parse_string_with_var1_alternative_token() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "welt".into());
        test_parse_helper(&key_value, "Hallo %(var1)", "Hallo welt");
    }

    #[test]
    fn test_parse_string_with_var1_and_var2_token() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "welt".into());
        key_value.insert("var2", "!!!!".into());
        test_parse_helper(&key_value, "Hallo %(var1)%(var2)", "Hallo welt!!!!");
    }

    #[test]
    fn test_parse_string_with_var1_and_var2_with_delimiter() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "a".into());
        key_value.insert("var2", "b".into());
        test_parse_helper(&key_value, "|%(var1)|%(var2)|", "|a|b|");
    }

    #[test]
    fn test_parse_string_with_var1_and_var2_var1_undefined() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "welt".into());
        test_parse_helper(&key_value, "Hallo %(var1)%(var2)", "Hallo welt");
    }

    #[test]
    fn test_parse_string_with_var1_and_var2_var2_undefined() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var2", "!!!!".into());
        test_parse_helper(&key_value, "Hallo %(var1)%(var2)", "Hallo !!!!");
    }

    #[test]
    fn test_parse_string_with_var1_invalid() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "welt".into());
        test_parse_helper(&key_value, "Hallo %var1", "Hallo ar1");
    }

    #[test]
    fn test_parse_string_with_var1_invalid2() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "welt".into());
        test_parse_helper(&key_value, "Hallo %(var1", "Hallo ");
    }

    #[test]
    fn test_parse_string_newline() {
        let key_value = HashMap::<&str, String>::new();
        test_parse_helper(&key_value, "Hallo %nWelt", "Hallo \nWelt");
    }

    #[test]
    fn test_parse_string_escape() {
        let key_value = HashMap::<&str, String>::new();
        test_parse_helper(&key_value, "Hallo %%(var1)", "Hallo %(var1)");
    }

    #[test]
    fn test_parse_string_newline_at_end() {
        let key_value = HashMap::<&str, String>::new();
        test_parse_helper(&key_value, "Hallo Welt %n", "Hallo Welt \n");
    }

    #[test]
    fn test_parse_string_format_specifier_left_align_and_token_smaller() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "1234".into());
        test_parse_helper(&key_value, "Hallo %<(10)%(var1)xx", "Hallo 1234      xx");
    }

    #[test]
    fn test_parse_string_format_specifier_left_align_and_token_exact() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "1234567890".into());
        test_parse_helper(&key_value, "Hallo %<(10)%(var1)xx", "Hallo 1234567890xx");
    }

    #[test]
    fn test_parse_string_format_specifier_left_align_and_token_bigger() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "1234567890ABCDEF".into());
        test_parse_helper(
            &key_value,
            "Hallo %<(10)%(var1)xx",
            "Hallo 1234567890ABCDEFxx",
        );
    }

    #[test]
    fn test_parse_string_format_specifier_left_align_and_token_invalid() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "1234567890ABCDEF".into());
        test_parse_helper(
            &key_value,
            "Hallo %<(a10)%(var1)xx",
            "Hallo %<(a10)1234567890ABCDEFxx",
        );
    }
}
