mod peek_char_iterator;

use std::collections::HashMap;

use self::peek_char_iterator::PeekCharIterator;

enum Format {
    None,
    LeftAlign(u32),
    LeftAlignTrunc(u32),
}

struct ParserContext<'a> {
    key_value: &'a HashMap<&'a str, String>,
    iter: &'a mut PeekCharIterator<'a>,
    vout: &'a mut Vec<char>,
    format: Format,
}

macro_rules! digit_pat {
    () => {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
    };
}

macro_rules! digit_without0_pat {
    () => {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
    };
}

macro_rules! consume_exp_chars{
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

macro_rules! collect_until_chars {
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
macro_rules! consume_until_not_char {
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

pub struct ExprParser;
impl ExprParser {
    pub fn new() -> Self {
        Self
    }

    fn consume_decimal(&self, context: &mut ParserContext<'_>) -> Option<u32> {
        let mut decimal_vec = Vec::<char>::new();

        let Some(first_digit) = consume_exp_chars!(context, digit_without0_pat!()) else {
            return None;
        };

        decimal_vec.push(first_digit);
        loop {
            let res_digit = consume_exp_chars!(context, digit_pat!());

            let Some(digit) = res_digit else {
                let decimal_str: String = decimal_vec.into_iter().collect();
                let decimal = decimal_str.parse::<u32>().unwrap();
                return Some(decimal);
            };

            decimal_vec.push(digit);
        }
    }

    fn interpret_named_placeholder(&self, context: &mut ParserContext<'_>) {
        let opt_literal = collect_until_chars!(context, ')');

        let Some(literal) = opt_literal else {
            context.vout.extend(context.iter.get_mark2cur().unwrap());
            return;
        };
        context.iter.next(); // consume ")"

        let literal_str: String = literal.into_iter().collect();

        let Some(value) = context.key_value.get(literal_str.as_str()) else {
            context.vout.extend(context.iter.get_mark2cur().unwrap());
            return;
        };

        match context.format {
            Format::LeftAlign(la) => {
                context.vout.extend(value.chars());
                let value_len = value.chars().count();
                let len_diff = (la as i32) - (value_len as i32);
                if len_diff > 0 {
                    for _i in 0..len_diff {
                        context.vout.push(' ')
                    }
                }
            }

            Format::LeftAlignTrunc(la) => {
                let value_len = value.chars().count();
                let len_diff = (la as i32) - (value_len as i32);

                match len_diff {
                    _ if len_diff > 0 => {
                        context.vout.extend(value.chars());
                        for _i in 0..len_diff {
                            context.vout.push(' ')
                        }
                    }

                    _ if len_diff < 0 => {
                        let let_cmp = (value_len as i32) + len_diff - 1;
                        for (idx, ch) in value.chars().enumerate() {
                            if idx >= let_cmp as usize {
                                break;
                            }
                            context.vout.push(ch);
                        }
                        context.vout.push('…');
                    }
                    _ => {
                        context.vout.extend(value.chars());
                    }
                }
            }
            _ => {
                context.vout.extend(value.chars());
            }
        }

        // Reset format for next Placeholder
        context.format = Format::None;
    }

    fn interpret_format_left(&self, context: &mut ParserContext<'_>) {
        if consume_exp_chars!(context, '(').is_none() {
            context.vout.extend(context.iter.get_mark2cur().unwrap());
            return;
        }
        consume_until_not_char!(context, ' '); // consume whitespaces

        let Some(decimal) = self.consume_decimal(context) else {
            context.vout.extend(context.iter.get_mark2cur().unwrap());
            return;
        };

        consume_until_not_char!(context, ' '); // consume whitespaces

        // Check if optional arguments are available
        if consume_exp_chars!(context, ',').is_some() {
            consume_until_not_char!(context, ' '); // consume whitespaces
            let Some(literal) = collect_until_chars!(context, ')') else {
                context.vout.extend(context.iter.get_mark2cur().unwrap());
                return;
            };
            context.iter.next(); // consume )
            let literal_str: String = literal.into_iter().collect();

            if literal_str.trim() == "trunc" {
                context.format = Format::LeftAlignTrunc(decimal);
                return;
            }
            //error
            context.vout.extend(context.iter.get_mark2cur().unwrap());
        } else {
            if consume_exp_chars!(context, ')').is_none() {
                context.vout.extend(context.iter.get_mark2cur().unwrap());
                return;
            }

            context.format = Format::LeftAlign(decimal);
        }
    }

    fn interpret_placeholder(&self, context: &mut ParserContext<'_>) {
        let Some(ch) = context.iter.next() else {
            return;
        };

        match ch {
            '(' => {
                self.interpret_named_placeholder(context);
            }
            '<' => {
                self.interpret_format_left(context);
            }
            'n' => {
                context.vout.push('\n');
            }
            '%' => {
                context.vout.push('%');
            }
            _ => {
                // error
                context.vout.extend(context.iter.get_mark2cur().unwrap());
            }
        }
    }

    pub fn parse(&self, key_value: &HashMap<&str, String>, inp: &str) -> String {
        let vec: Vec<_> = inp.chars().collect();
        let mut iter = PeekCharIterator::new(&vec);
        let mut vout = Vec::<char>::new();
        let mut context = ParserContext {
            key_value,
            iter: &mut iter,
            vout: &mut vout,
            format: Format::None,
        };

        loop {
            let Some(ch) = context.iter.peek() else {
                break;
            };

            match ch {
                '%' => {
                    context.iter.mark(); // mark position of placeholder start
                    context.iter.next();
                    self.interpret_placeholder(&mut context);
                }
                _ => {
                    context.iter.next();
                    context.vout.push(ch)
                }
            }
        }
        vout.into_iter().collect()
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
