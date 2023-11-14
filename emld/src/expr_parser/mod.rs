mod output_format;
mod peek_char_iterator;
mod type_trait;
use self::output_format::OutputFormat;
use self::peek_char_iterator::PeekCharIterator;
use self::type_trait::{CharType, TypeTrait};
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
        consume_expected_chars!(
            $context,
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
        )
    };
}

macro_rules! consume_digits_without_0 {
    ($context:ident) => {
        consume_expected_chars!(
            $context,
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
        )
    };
}

macro_rules! gather_until_match {
    ($context:ident, $($a:pat)+) => {{
        let mut vec: Vec<char> = Vec::new();
        loop {
            let Some(ch) = $context.iter.peek() else {
                break None;
            };

            match ch {
                $($a)|+ => {
                    break Some(vec);
                }
                _ => {
                    $context.iter.next();
                vec.push(ch);
                }
            }
        }
    }};
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

struct ParsingContext<'a, T: TypeTrait> {
    key_value: &'a HashMap<&'a str, String>,
    iter: PeekCharIterator,
    vout: Vec<T::Item>,
    format: OutputFormat,
}

pub struct ExpressionParser;

struct ParsingTaskStringInterpolation;
trait ParsingTask {
    type Item: TypeTrait;
    type Output;

    /// Called in case the context should be initialized
    fn init<'a>(
        inp: &'a str,
        key_value: &'a HashMap<&'a str, String>,
    ) -> ParsingContext<'a, Self::Item>;

    fn done(context: ParsingContext<'_, Self::Item>) -> Self::Output;

    /// Called in case that the parser encounts an error
    fn error(context: &mut ParsingContext<'_, Self::Item>);

    /// Called in case that the input should be copied to the output
    fn mirror(context: &mut ParsingContext<'_, Self::Item>, ch: char);

    /// Called in case that a single char placeholder should be substitutet
    fn output_char_placeholder(context: &mut ParsingContext<'_, Self::Item>, ch: char);

    /// Called in case that a placeholder should be substitutet
    fn output_placeholder(context: &mut ParsingContext<'_, Self::Item>, arg: String);
}

impl ParsingTask for ParsingTaskStringInterpolation {
    type Item = CharType;
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

    /// Called in case that the parser encounts an error
    fn error(context: &mut ParsingContext<'_, Self::Item>) {
        context.vout.extend(context.iter.get_mark2cur().unwrap());
    }

    /// Called in case that the input should be copied to the output
    fn mirror(context: &mut ParsingContext<'_, Self::Item>, ch: char) {
        context.vout.push(ch);
    }

    /// Called in case that a single char placeholder should be substitutet
    fn output_char_placeholder(context: &mut ParsingContext<'_, Self::Item>, ch: char) {
        context.vout.push(ch);
    }

    /// Called in case that a placeholder should be substitutet
    fn output_placeholder(context: &mut ParsingContext<'_, Self::Item>, arg: String) {
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

    fn parse_decimal_number<I: TypeTrait>(
        &self,
        context: &mut ParsingContext<'_, I>,
    ) -> Option<u32> {
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

    fn process_named_placeholder<T: ParsingTask>(&self, context: &mut ParsingContext<'_, T::Item>) {
        let opt_literal = gather_until_match!(context, ')');

        let Some(literal) = opt_literal else {
            T::error(context);
            return;
        };
        context.iter.next(); // consume ")"

        T::output_placeholder(context, literal.into_iter().collect());

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
            let Some(literal) = gather_until_match!(context, ')') else {
                T::error(context);
                return;
            };
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
                self.process_named_placeholder::<T>(context);
            }
            '<' => {
                self.process_format_left_placeholder::<T>(context);
            }
            'n' => {
                T::output_char_placeholder(context, '\n');
            }
            '%' => {
                T::output_char_placeholder(context, '%');
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
                    T::mirror(&mut context, ch);
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
mod tests {
    use std::collections::HashMap;

    use crate::expr_parser::ExpressionParser;

    fn test_parse_helper(key_value: &HashMap<&str, String>, inp: &str, expected_output: &str) {
        let parser = ExpressionParser::new();

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
    fn test_parse_string_wrong_token_type() {
        let key_value = HashMap::<&str, String>::new();
        test_parse_helper(&key_value, "Hallo %z", "Hallo %z");
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
        test_parse_helper(&key_value, "Hallo %(var1)%(var2)", "Hallo welt%(var2)");
    }

    #[test]
    fn test_parse_string_with_var1_and_var2_var2_undefined() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var2", "!!!!".into());
        test_parse_helper(&key_value, "Hallo %(var1)%(var2)", "Hallo %(var1)!!!!");
    }

    #[test]
    fn test_parse_string_with_var1_invalid() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "welt".into());
        test_parse_helper(&key_value, "Hallo %var1", "Hallo %var1");
    }

    #[test]
    fn test_parse_string_with_var1_invalid2() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "welt".into());
        test_parse_helper(&key_value, "Hallo %(var1", "Hallo %(var1");
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
    fn test_parse_string_format_specifier_left_align_with_trunc_and_token_exact() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "1234567890".into());
        test_parse_helper(
            &key_value,
            "Hallo %<(10,trunc)%(var1)xx",
            "Hallo 1234567890xx",
        );
    }

    #[test]
    fn test_parse_string_format_specifier_left_align_with_trunc_and_token_exact_spaces() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "1234567890".into());
        test_parse_helper(
            &key_value,
            "Hallo %<(  10  ,  trunc   )%(var1)xx",
            "Hallo 1234567890xx",
        );
    }

    #[test]
    fn test_parse_string_format_specifier_left_align_with_trunc_and_token_bigger() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "1234567890ABCDEF".into());
        test_parse_helper(
            &key_value,
            "Hallo %<(10,trunc)%(var1)xx",
            "Hallo 123456789…xx",
        );
    }

    #[test]
    fn test_parse_string_format_specifier_left_align_with_trunc_and_token_smaller_umlaute() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "äöü".into());
        test_parse_helper(
            &key_value,
            "Hallo %<(10,trunc)%(var1)xx",
            "Hallo äöü       xx",
        );
    }

    #[test]
    fn test_parse_string_format_specifier_left_align_with_trunc_and_token_biggerer_umlaute() {
        let mut key_value = HashMap::<&str, String>::new();
        key_value.insert("var1", "äöü12345678".into());
        test_parse_helper(
            &key_value,
            "Hallo %<(10,trunc)%(var1)xx",
            "Hallo äöü123456…xx",
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