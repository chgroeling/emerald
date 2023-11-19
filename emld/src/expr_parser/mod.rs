mod output_format;
mod parsing_context;
mod parsing_task;
mod parsing_task_analyze;
mod parsing_task_format;
mod parsing_task_measure;
mod peek_char_iterator;

use self::output_format::OutputFormat;
use self::parsing_context::ParsingContext;
use self::parsing_task::ParsingTask;
use self::parsing_task_analyze::ParsingTaskAnalyze;
use self::parsing_task_format::ParsingTaskFormat;
use self::parsing_task_measure::ParsingTaskMeasure;
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

pub struct ExpressionParser;

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

    pub fn format(&self, key_value: &HashMap<&str, String>, inp: &str) -> String {
        self.parse_generic::<ParsingTaskFormat>(key_value, inp)
    }

    pub fn measure(&self, key_value: &HashMap<&str, String>, inp: &str) -> Vec<usize> {
        self.parse_generic::<ParsingTaskMeasure>(key_value, inp)
    }

    #[allow(unused_imports, dead_code)]
    pub fn analyze(&self, key_value: &HashMap<&str, String>, inp: &str) -> Vec<String> {
        self.parse_generic::<ParsingTaskAnalyze>(key_value, inp)
    }
}

#[cfg(test)]
mod tests_analyze {
    use std::collections::HashMap;

    use crate::expr_parser::ExpressionParser;

    macro_rules! test {
        ($test_name:ident, $inp:expr, $expected_output:expr) => {
            #[test]
            fn $test_name() {
                let mut key_value = HashMap::<&str, String>::new();
                key_value.insert("var1", "world".into());
                key_value.insert("var2", "welt".into());
                key_value.insert("str4", "1234".into());
                key_value.insert("str10", "1234567890".into());
                key_value.insert("str14", "1234567890ABCD".into());
                key_value.insert("umlaute", "äöü".into());
                key_value.insert("umlaute_bigger", "äöü12345678".into());
                let parser = ExpressionParser::new();
                let out_str = parser.analyze(&key_value, $inp);
                assert_eq!(out_str, $expected_output);
            }
        };
    }

    test!(
        test_analyze_with_empty_input_returns_empty_vec,
        "",
        Vec::<String>::new()
    );

    test!(
        test_analyze_with_plain_string_returns_empty_vec,
        "Conventional string",
        Vec::<String>::new()
    );

    test!(
        test_analyze_with_unicode_string_returns_empty_vec,
        "Smiley 😊 Smiley",
        Vec::<String>::new()
    );

    test!(
        test_analyze_with_single_placeholder_returns_one_placeholder,
        "Hello %(var1)", // replaces to "Hello world"
        vec!["var1"]
    );

    test!(
        test_analyze_with_multiple_placeholders_return_two_placeholders,
        "Hello %(var1). Hallo %(var2).", // "Hello world. Hallo welt."
        vec!["var1", "var2"]
    );

    test!(
        test_analyze_with_undefined_second_placeholder_returns_two_placeholders,
        "Hallo %(var1)%(vara)",
        vec!["var1"]
    );

    test!(
        test_analyze_with_incomplete_placeholder_syntax_returns_empty_vec,
        "Hallo %(var1",
        Vec::<String>::new()
    );
}

#[cfg(test)]
mod tests_measure {
    use std::collections::HashMap;

    use crate::expr_parser::ExpressionParser;

    macro_rules! test {
        ($test_name:ident, $inp:expr, $expected_output:expr) => {
            #[test]
            fn $test_name() {
                let mut key_value = HashMap::<&str, String>::new();
                key_value.insert("var1", "world".into());
                key_value.insert("var2", "welt".into());
                key_value.insert("str4", "1234".into());
                key_value.insert("str10", "1234567890".into());
                key_value.insert("str14", "1234567890ABCD".into());
                key_value.insert("umlaute", "äöü".into());
                key_value.insert("umlaute_bigger", "äöü12345678".into());
                let parser = ExpressionParser::new();
                let out_str = parser.measure(&key_value, $inp);
                assert_eq!(out_str, $expected_output);
            }
        };
    }

    test!(test_measure_with_empty_input_returns_vec0, "", vec![0usize]);
    test!(
        test_measure_with_plain_string_returns_correct_length,
        "Conventional string",
        vec![19usize]
    );

    test!(
        test_measure_with_unicode_string_returns_correct_length,
        "Smiley 😊 Smiley",
        vec![15usize]
    );

    test!(
        test_measure_with_single_placeholder_measures_correctly,
        "Hello %(var1)", // replaces to "Hello world"
        vec![11usize, 5usize]
    );

    test!(
        test_measure_with_invalid_token_type_counts_length_of_unreplaced_string,
        "Hallo %z", // replaces nothing
        vec![8usize]
    );

    test!(
        test_measure_with_multiple_placeholders_return_correct_length_of_string_and_placeholders,
        "Hello %(var1). Hallo %(var2).", // "Hello world. Hallo welt."
        vec![24usize, 5usize, 4usize]
    );

    test!(
        test_measure_with_undefined_second_placeholder_return_correct_length_of_string_and_placeholders,
        "Hallo %(var1)%(vara)", // "Hallo world%(vara)"
        vec![18usize, 5usize]
    );

    test!(
        test_measure_with_left_alignment_placeholder_and_shorter_value_returns_correct_length,
        "Hallo %<(10)%(str4)xx", // "Hallo 1234      xx"
        vec![18usize, 10usize]
    );

    test!(
        test_measure_with_left_alignment_placeholder_and_exact_length_value_returns_correct_length,
        "Hallo %<(10)%(str10)xx", // "Hallo 1234567890xx"
        vec![18usize, 10usize]
    );

    test!(
        test_measure_with_left_alignment_placeholder_and_longer_value_returns_correct_length,
        "Hallo %<(10)%(str14)xx", // "Hallo 1234567890ABCDxx"
        vec![22usize, 14usize]
    );

    test!(
        test_measure_with_left_align_truncate_placeholder_and_shorter_value_with_umlauts_returns_correct_length,
        "Hallo %<(10,trunc)%(umlaute)xx", // "Hallo äöü       xx"
        vec![18usize, 10usize]
    );

    test!(
        test_measure_with_left_align_truncate_placeholder_and_exact_length_value_returns_correct_length,
        "Hallo %<(10,trunc)%(str10)xx", // "Hallo 1234567890xx"
        vec![18usize, 10usize]
    );

    test!(
        test_measure_with_left_align_truncate_placeholder_and_longer_value_returns_correct_length,
        "Hallo %<(10,trunc)%(str14)xx", // "Hallo 123456789…xx"
        vec![18usize, 10usize]
    );
}

#[cfg(test)]
mod tests_format {
    use crate::expr_parser::ExpressionParser;
    use std::collections::HashMap;

    macro_rules! test {
        ($test_name:ident, $inp:expr, $expected_output:expr) => {
            #[test]
            fn $test_name() {
                let mut key_value = HashMap::<&str, String>::new();
                key_value.insert("var1", "world".into());
                key_value.insert("var2", "welt".into());
                key_value.insert("str4", "1234".into());
                key_value.insert("str10", "1234567890".into());
                key_value.insert("str14", "1234567890ABCD".into());
                key_value.insert("umlaute", "äöü".into());
                key_value.insert("umlaute_bigger", "äöü12345678".into());
                let parser = ExpressionParser::new();
                let out_str = parser.format(&key_value, $inp);
                assert_eq!(out_str, $expected_output);
            }
        };
    }

    test!(test_format_with_empty_input_returns_empty_string, "", "");

    test!(
        test_format_with_plain_string_returns_same_string,
        "Conventional string",
        "Conventional string"
    );

    test!(
        test_format_with_unicode_string_returns_same_string,
        "Smiley 😊 Smiley",
        "Smiley 😊 Smiley"
    );

    test!(
        test_format_with_single_placeholder_replaces_correctly,
        "Hello %(var1)",
        "Hello world"
    );

    test!(
        test_format_with_single_placeholder_alternative_value_replaces_correctly,
        "Hello %(var2)",
        "Hello welt"
    );

    test!(
        test_format_with_invalid_token_type_leaves_token_unreplaced,
        "Hallo %z",
        "Hallo %z"
    );

    test!(
        test_format_with_multiple_placeholders_replaces_all_correctly,
        "Hello %(var1). Hallo %(var2).",
        "Hello world. Hallo welt."
    );

    test!(
        test_format_with_multiple_placeholders_and_delimiters_replaces_correctly,
        "|%(var1)|%(var2)|",
        "|world|welt|"
    );

    test!(
        test_format_with_undefined_second_placeholder_keeps_it_unreplaced,
        "Hallo %(var1)%(vara)",
        "Hallo world%(vara)"
    );

    test!(
        test_format_with_undefined_first_placeholder_keeps_it_unreplaced,
        "Hallo %(vara)%(var2)",
        "Hallo %(vara)welt"
    );

    test!(
        test_format_with_incorrect_placeholder_syntax_keeps_it_unreplaced,
        "Hallo %var1",
        "Hallo %var1"
    );

    test!(
        test_format_with_incomplete_placeholder_syntax_keeps_it_unreplaced,
        "Hallo %(var1",
        "Hallo %(var1"
    );

    test!(
        test_format_with_newline_placeholder_inserts_newline,
        "Hallo %nWelt",
        "Hallo \nWelt"
    );

    test!(
        test_format_with_escaped_percent_sign_keeps_it_unchanged,
        "Hallo %%(var1)",
        "Hallo %(var1)"
    );

    test!(
        test_format_with_newline_placeholder_at_end_inserts_newline,
        "Hallo Welt %n",
        "Hallo Welt \n"
    );

    test!(
        test_format_with_left_alignment_placeholder_and_shorter_value_pads_correctly,
        "Hallo %<(10)%(str4)xx",
        "Hallo 1234      xx"
    );

    test!(
        test_format_with_left_alignment_placeholder_and_exact_length_value_keeps_it_unchanged,
        "Hallo %<(10)%(str10)xx",
        "Hallo 1234567890xx"
    );

    test!(
        test_format_with_left_alignment_placeholder_and_longer_value_keeps_it_unchanged,
        "Hallo %<(10)%(str14)xx",
        "Hallo 1234567890ABCDxx"
    );

    test!(
        test_format_with_left_align_truncate_placeholder_and_exact_length_value_keeps_it_unchanged,
        "Hallo %<(10,trunc)%(str10)xx",
        "Hallo 1234567890xx"
    );

    test!(
        test_format_with_left_align_truncate_placeholder_and_exact_length_value_with_spaces_keeps_it_unchanged,
        "Hallo %<(  10  ,  trunc   )%(str10)xx",
        "Hallo 1234567890xx"
    );

    test!(
        test_format_with_left_align_truncate_placeholder_and_longer_value_truncates_correctly,
        "Hallo %<(10,trunc)%(str14)xx",
        "Hallo 123456789…xx"
    );

    test!(
        test_format_with_left_align_truncate_placeholder_and_shorter_value_with_umlauts_pads_correctly,
        "Hallo %<(10,trunc)%(umlaute)xx",
        "Hallo äöü       xx"
    );

    test!(
        test_format_with_left_align_truncate_placeholder_and_longer_value_with_umlauts_truncates_correctly,
        "Hallo %<(10,trunc)%(umlaute_bigger)xx",
        "Hallo äöü123456…xx"
    );

    test!(
        test_format_with_invalid_left_align_placeholder_keeps_format_specifier_unchanged,
        "Hallo %<(a10)%(str14)xx",
        "Hallo %<(a10)1234567890ABCDxx"
    );
}
