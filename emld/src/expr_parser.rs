use std::collections::HashMap;

enum Format {
    None,
    LeftAlign(u32),
}

struct ParserContext<'a, I>
where
    I: Iterator<Item = char>,
{
    key_value: &'a HashMap<&'a str, String>,
    iter: &'a mut I,
    out_str: &'a mut Vec<char>,
    format: Format,
}

struct ParseResult<I> {
    result: Option<I>,
    last_char: char,
}

impl<I> ParseResult<I> {
    fn new(result: Option<I>, last_char: char) -> Self {
        Self {
            result: result,
            last_char: last_char,
        }
    }
}
pub struct ExprParser;
impl ExprParser {
    pub fn new() -> Self {
        Self
    }
    fn interpret_named_placeholder<I>(&self, context: &mut ParserContext<'_, I>)
    where
        I: Iterator<Item = char>,
    {
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

    fn consume_digit_without_0<I>(&self, context: &mut ParserContext<'_, I>) -> ParseResult<char>
    where
        I: Iterator<Item = char>,
    {
        loop {
            let Some(ch) = context.iter.next() else {
                return ParseResult::new(None, '\0');
            };

            match ch {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    return ParseResult::new(Some(ch), ch)
                }
                _ => {
                    return ParseResult::new(None, ch);
                }
            }
        }
    }

    fn consume_digit<I>(&self, context: &mut ParserContext<'_, I>) -> ParseResult<char>
    where
        I: Iterator<Item = char>,
    {
        loop {
            let Some(ch) = context.iter.next() else {
                return ParseResult::new(None, '\0');
            };

            match ch {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    return ParseResult::new(Some(ch), ch)
                }
                _ => return ParseResult::new(None, ch),
            }
        }
    }

    fn consume_expected_char<I>(
        &self,
        context: &mut ParserContext<'_, I>,
        expected_char: char,
    ) -> ParseResult<char>
    where
        I: Iterator<Item = char>,
    {
        loop {
            let Some(ch) = context.iter.next() else {
                return ParseResult::new(None, '\0');
            };

            if ch == expected_char {
                return ParseResult::new(Some(ch), ch);
            } else {
                return ParseResult::new(None, ch);
            }
        }
    }

    fn consume_decimal<I>(&self, context: &mut ParserContext<'_, I>) -> ParseResult<u32>
    where
        I: Iterator<Item = char>,
    {
        let mut decimal_vec = Vec::<char>::new();

        let res_first_digit = self.consume_digit_without_0(context);
        let Some(first_digit) = res_first_digit.result else {
            return ParseResult::new(None, res_first_digit.last_char);
        };

        decimal_vec.push(first_digit);
        loop {
            let res_digit = self.consume_digit(context);

            let Some(digit) = res_digit.result else {
                let decimal_str: String = decimal_vec.into_iter().collect();
                let decimal = decimal_str.parse::<u32>().unwrap();
                return ParseResult::new(Some(decimal), res_digit.last_char);
            };

            decimal_vec.push(digit);
        }
    }

    fn interpret_format_open<I>(&self, context: &mut ParserContext<'_, I>)
    where
        I: Iterator<Item = char>,
    {
        let res_open = self.consume_expected_char(context, '(');
        if res_open.result.is_none() {
            return;
        }
        let res_decimal = self.consume_decimal(context);
        if res_decimal.last_char != ')' {
            return;
        }

        let Some(decimal) = res_decimal.result else {
            return;
        };
        context.format = Format::LeftAlign(decimal);
    }

    fn interpret_placeholder<I>(&self, context: &mut ParserContext<'_, I>)
    where
        I: Iterator<Item = char>,
    {
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
        let mut iter = inp.chars();
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
}
