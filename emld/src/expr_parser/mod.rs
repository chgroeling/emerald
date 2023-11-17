mod output_format;
mod peek_char_iterator;
use self::output_format::OutputFormat;
use self::peek_char_iterator::PeekCharIterator;
use std::collections::HashMap;

/// `consume_expected_chars` checks and consumes the next char in the iterator if it matches the provided pattern(s).
/// - `$context`: The parsing context containing the `PeekCharIterator`.
/// - `$($a:pat)+`: Pattern(s) to match against the next char.
/// If the next char matches, it's consumed and returned as `Some(char)`. Otherwise, returns `None`.
macro_rules! consume_expected_chars{
    ($context:ident, $($a:pat)+) => {
        if let Some(ch) = $context.iter.peek()  {
            match ch {
                $($a)|+ => {
                    $context.iter.next(); // consume
                    Some(ch)
                }
                _ => {
                    None
                }
            }
        } else {
            None
        }
    };
}

macro_rules! consume_digits {
    ($context:ident) => {
        consume_expected_chars!($context, '0'..='9')
    };
}

macro_rules! consume_digits_without_0 {
    ($context:ident) => {
        consume_expected_chars!($context, '1'..='9')
    };
}

macro_rules! gather {
    ($context:ident, $($a:pat)+) => {{
        let mut vec: Vec<char> = Vec::new();
        loop {
            let Some(ch) = $context.iter.peek() else {
                break None;
            };

            match ch {
                $($a)|+ => {
                    vec.push(ch);
                    $context.iter.next();

                }
                _ => {
                    break Some(vec);
                }
            }
        }
    }};
}

macro_rules! gather_str_placeholder {
    ($context:ident) => {
        gather!(
            $context,
            ('0'..='9')
                | ('a'..='z')
                | ('A'..='Z')
                | '_'
                | '+'
                | '*'
                | '/'
                | 'ä'
                | 'ö'
                | 'ü'
                | 'ß'
                | '?'
        )
    };
}

macro_rules! skip_until_neg_char_match {
    ($context:ident, $a:expr) => {
        loop {
            let Some(ch) = $context.iter.peek() else {
                break None;
            };

            if ch != $a {
                break Some(());
            } else {
                $context.iter.next();
            }
        }
    };
}

struct ParsingContext<'a, T> {
    key_value: &'a HashMap<&'a str, String>,
    iter: PeekCharIterator,
    vout: Vec<T>,
    format: OutputFormat,
}

pub struct ExpressionParser;

struct ParsingTaskStringInterpolation;
trait ParsingTask {
    type Item;
    type Output;

    /// Initializes the parsing context at the start of parsing.
    fn init<'a>(
        inp: &'a str,
        key_value: &'a HashMap<&'a str, String>,
    ) -> ParsingContext<'a, Self::Item>;

    /// Finalizes the parsing process.
    fn done(context: ParsingContext<'_, Self::Item>) -> Self::Output;

    /// Handles errors encountered during parsing.
    fn error(context: &mut ParsingContext<'_, Self::Item>);

    /// Copies a character from the input to the output as is.
    fn process_char(context: &mut ParsingContext<'_, Self::Item>, ch: char);

    /// Processes a single character placeholder.
    fn process_char_placeholder(context: &mut ParsingContext<'_, Self::Item>, ch: char);

    /// Processes a placeholder represented by a string.
    fn process_str_placeholder(context: &mut ParsingContext<'_, Self::Item>, arg: String);
}

impl ParsingTask for ParsingTaskStringInterpolation {
    type Item = char;
    type Output = String;

    /// Called in case the context should be initialized
    fn init<'a>(
        inp: &'a str,
        key_value: &'a HashMap<&'a str, String>,
    ) -> ParsingContext<'a, Self::Item> {
        let vec: Vec<_> = inp.chars().collect();
        ParsingContext::<'_, Self::Item> {
            key_value,
            iter: PeekCharIterator::new(vec),
            vout: Vec::<char>::new(),
            format: OutputFormat::None,
        }
    }

    fn error(context: &mut ParsingContext<'_, Self::Item>) {
        context.vout.extend(context.iter.get_mark2cur().unwrap());
    }

    fn process_char(context: &mut ParsingContext<'_, Self::Item>, ch: char) {
        context.vout.push(ch);
    }

    fn process_char_placeholder(context: &mut ParsingContext<'_, Self::Item>, ch: char) {
        context.vout.push(ch);
    }

    fn process_str_placeholder(context: &mut ParsingContext<'_, Self::Item>, arg: String) {
        let Some(repl_str) = context.key_value.get(arg.as_str()) else {
            Self::error(context);
            return;
        };
        let repl = repl_str.chars();
        match context.format {
            OutputFormat::LeftAlign(la) => {
                context.vout.extend(repl.clone());
                let value_len = repl.into_iter().count();
                let len_diff = (la as i32) - (value_len as i32);
                if len_diff > 0 {
                    for _i in 0..len_diff {
                        context.vout.push(' ');
                    }
                }
            }

            OutputFormat::LeftAlignTrunc(la) => {
                let value_len = repl.clone().count();
                let len_diff = (la as i32) - (value_len as i32);

                match len_diff {
                    _ if len_diff > 0 => {
                        context.vout.extend(repl);
                        for _i in 0..len_diff {
                            context.vout.push(' ');
                        }
                    }

                    _ if len_diff < 0 => {
                        let let_cmp = (value_len as i32) + len_diff - 1;
                        for (idx, ch) in repl.into_iter().enumerate() {
                            if idx >= let_cmp as usize {
                                break;
                            }
                            context.vout.push(ch);
                        }
                        context.vout.push('…');
                    }
                    _ => {
                        context.vout.extend(repl);
                    }
                }
            }
            _ => {
                context.vout.extend(repl);
            }
        }
    }

    fn done(context: ParsingContext<'_, Self::Item>) -> Self::Output {
        context.vout.into_iter().collect()
    }
}

impl ExpressionParser {
    pub fn new() -> Self {
        Self
    }

    fn parse_decimal_number<I>(&self, context: &mut ParsingContext<'_, I>) -> Option<u32> {
        let mut decimal_vec = Vec::<char>::new();

        let Some(first_digit) = consume_digits_without_0!(context) else {
            return None;
        };

        decimal_vec.push(first_digit);
        loop {
            let res_digit = consume_digits!(context);

            let Some(digit) = res_digit else {
                let decimal_str: String = decimal_vec.into_iter().collect();
                let decimal = decimal_str.parse::<u32>().unwrap();
                return Some(decimal);
            };

            decimal_vec.push(digit);
        }
    }

    fn process_str_placeholder<T: ParsingTask>(&self, context: &mut ParsingContext<'_, T::Item>) {
        let opt_literal = gather_str_placeholder!(context);

        let Some(literal) = opt_literal else {
            T::error(context);
            return;
        };
        context.iter.next(); // consume ")"

        T::process_str_placeholder(context, literal.into_iter().collect());

        // Reset format for next Placeholder
        context.format = OutputFormat::None;
    }

    fn process_format_left_placeholder<T: ParsingTask>(
        &self,
        context: &mut ParsingContext<'_, T::Item>,
    ) {
        if consume_expected_chars!(context, '(').is_none() {
            T::error(context);
            return;
        }
        skip_until_neg_char_match!(context, ' '); // consume whitespaces

        let Some(decimal) = self.parse_decimal_number(context) else {
            T::error(context);
            return;
        };

        skip_until_neg_char_match!(context, ' '); // consume whitespaces

        // Check if optional arguments are available
        if consume_expected_chars!(context, ',').is_some() {
            skip_until_neg_char_match!(context, ' '); // consume whitespaces
            let Some(literal) = gather_str_placeholder!(context) else {
                T::error(context);
                return;
            };
            skip_until_neg_char_match!(context, ' '); // consume whitespaces
            context.iter.next(); // consume )
            let arg: String = literal.into_iter().collect();

            if arg.trim() == "trunc" {
                context.format = OutputFormat::LeftAlignTrunc(decimal);
                return;
            }

            T::error(context);
        } else {
            if consume_expected_chars!(context, ')').is_none() {
                T::error(context);
                return;
            }

            context.format = OutputFormat::LeftAlign(decimal);
        }
    }

    fn process_placeholder<T: ParsingTask>(&self, context: &mut ParsingContext<'_, T::Item>) {
        let Some(ch) = context.iter.next() else {
            return;
        };

        match ch {
            '(' => {
                self.process_str_placeholder::<T>(context);
            }
            '<' => {
                self.process_format_left_placeholder::<T>(context);
            }
            'n' => {
                T::process_char_placeholder(context, '\n');
            }
            '%' => {
                T::process_char_placeholder(context, '%');
            }
            _ => {
                T::error(context);
            }
        }
    }

    fn parse_generic<T: ParsingTask>(
        &self,
        key_value: &HashMap<&str, String>,
        inp: &str,
    ) -> T::Output {
        let mut context = T::init(inp, key_value);
        loop {
            let Some(ch) = context.iter.peek() else {
                break;
            };

            match ch {
                '%' => {
                    context.iter.mark(); // mark position of placeholder start
                    context.iter.next();
                    self.process_placeholder::<T>(&mut context);
                }
                _ => {
                    context.iter.next();
                    T::process_char(&mut context, ch);
                }
            }
        }
        T::done(context)
    }

    pub fn parse(&self, key_value: &HashMap<&str, String>, inp: &str) -> String {
        self.parse_generic::<ParsingTaskStringInterpolation>(key_value, inp)
    }
}
#[cfg(test)]
mod tests_info {}

#[cfg(test)]
mod tests_interplation {
    use std::collections::HashMap;

    use crate::expr_parser::ExpressionParser;

    fn test_parse_helper(key_value: &HashMap<&str, String>, inp: &str, expected_output: &str) {
        let parser = ExpressionParser::new();

        let out_str = parser.parse(&key_value, inp);
        assert_eq!(out_str, expected_output);
    }

    #[test]
    fn test_parse_with_empty_input_returns_empty_string() {
        let key_value = HashMap::<&str, String>::new();
        test_parse_helper(&key_value, "", "");
    }

    #[test]
    fn test_parse_with_plain_string_returns_same_string() {
        let key_value = HashMap::<&str, String>::new();
        test_parse_helper(&key_value, "Conventional string", "Conventional string");
    }

    #[test]
    fn test_parse_with_unicode_string_returns_same_string() {
        let key_value = HashMap::<&str, String>::new();
        test_parse_helper(&key_value, "Smiley 😊 Smiley", "Smiley 😊 Smiley");
    }

    #[test]
    fn test_parse_with_single_placeholder_replaces_correctly() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "world".into());
        test_parse_helper(&key_value, "Hello %(var1)", "Hello world");
    }

    #[test]
    fn test_parse_with_single_placeholder_alternative_value_replaces_correctly() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "welt".into());
        test_parse_helper(&key_value, "Hallo %(var1)", "Hallo welt");
    }

    #[test]
    fn test_parse_with_invalid_token_type_leaves_token_unreplaced() {
        let key_value = HashMap::<&str, String>::new();
        test_parse_helper(&key_value, "Hallo %z", "Hallo %z");
    }

    #[test]
    fn test_parse_with_multiple_placeholders_replaces_all_correctly() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "welt".into());
        key_value.insert("var2", "!!!!".into());
        test_parse_helper(&key_value, "Hallo %(var1)%(var2)", "Hallo welt!!!!");
    }

    #[test]
    fn test_parse_with_multiple_placeholders_and_delimiters_replaces_correctly() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "a".into());
        key_value.insert("var2", "b".into());
        test_parse_helper(&key_value, "|%(var1)|%(var2)|", "|a|b|");
    }

    #[test]
    fn test_parse_with_undefined_first_placeholder_keeps_it_unreplaced() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var2", "!!!!".into());
        test_parse_helper(&key_value, "Hallo %(var1)%(var2)", "Hallo %(var1)!!!!");
    }

    #[test]
    fn test_parse_with_undefined_second_placeholder_keeps_it_unreplaced() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "welt".into());
        test_parse_helper(&key_value, "Hallo %(var1)%(var2)", "Hallo welt%(var2)");
    }

    #[test]
    fn test_parse_with_incorrect_placeholder_syntax_keeps_it_unreplaced() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "welt".into());
        test_parse_helper(&key_value, "Hallo %var1", "Hallo %var1");
    }

    #[test]
    fn test_parse_with_incomplete_placeholder_syntax_keeps_it_unreplaced() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "welt".into());
        test_parse_helper(&key_value, "Hallo %(var1", "Hallo %(var1");
    }

    #[test]
    fn test_parse_with_newline_placeholder_inserts_newline() {
        let key_value = HashMap::<&str, String>::new();
        test_parse_helper(&key_value, "Hallo %nWelt", "Hallo \nWelt");
    }

    #[test]
    fn test_parse_with_escaped_percent_sign_keeps_it_unchanged() {
        let key_value = HashMap::<&str, String>::new();
        test_parse_helper(&key_value, "Hallo %%(var1)", "Hallo %(var1)");
    }

    #[test]
    fn test_parse_with_newline_placeholder_at_end_inserts_newline() {
        let key_value = HashMap::<&str, String>::new();
        test_parse_helper(&key_value, "Hallo Welt %n", "Hallo Welt \n");
    }

    #[test]
    fn test_parse_with_left_alignment_and_shorter_value_pads_correctly() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "1234".into());
        test_parse_helper(&key_value, "Hallo %<(10)%(var1)xx", "Hallo 1234      xx");
    }

    #[test]
    fn test_parse_with_left_alignment_and_exact_length_value_keeps_it_unchanged() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "1234567890".into());
        test_parse_helper(&key_value, "Hallo %<(10)%(var1)xx", "Hallo 1234567890xx");
    }

    #[test]
    fn test_parse_with_left_alignment_and_longer_value_keeps_it_unchanged() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "1234567890ABCDEF".into());
        test_parse_helper(
            &key_value,
            "Hallo %<(10)%(var1)xx",
            "Hallo 1234567890ABCDEFxx",
        );
    }

    #[test]
    fn test_parse_with_left_align_truncate_and_exact_length_value_keeps_it_unchanged() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "1234567890".into());
        test_parse_helper(
            &key_value,
            "Hallo %<(10,trunc)%(var1)xx",
            "Hallo 1234567890xx",
        );
    }

    #[test]
    fn test_parse_with_left_align_truncate_and_exact_length_value_with_spaces_keeps_it_unchanged() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "1234567890".into());
        test_parse_helper(
            &key_value,
            "Hallo %<(  10  ,  trunc   )%(var1)xx",
            "Hallo 1234567890xx",
        );
    }

    #[test]
    fn test_parse_with_left_align_truncate_and_longer_value_truncates_correctly() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "1234567890ABCDEF".into());
        test_parse_helper(
            &key_value,
            "Hallo %<(10,trunc)%(var1)xx",
            "Hallo 123456789…xx",
        );
    }

    #[test]
    fn test_parse_with_left_align_truncate_and_shorter_value_with_umlauts_pads_correctly() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "äöü".into());
        test_parse_helper(
            &key_value,
            "Hallo %<(10,trunc)%(var1)xx",
            "Hallo äöü       xx",
        );
    }

    #[test]
    fn test_parse_with_left_align_truncate_and_longer_value_with_umlauts_truncates_correctly() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "äöü12345678".into());
        test_parse_helper(
            &key_value,
            "Hallo %<(10,trunc)%(var1)xx",
            "Hallo äöü123456…xx",
        );
    }

    #[test]
    fn test_parse_with_invalid_left_align_argument_keeps_format_specifier_unchanged() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "1234567890ABCDEF".into());
        test_parse_helper(
            &key_value,
            "Hallo %<(a10)%(var1)xx",
            "Hallo %<(a10)1234567890ABCDEFxx",
        );
    }
}
