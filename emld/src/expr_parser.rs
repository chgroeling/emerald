use std::collections::HashMap;

pub struct ExprParser;
impl ExprParser {
    pub fn new() -> Self {
        Self
    }
    pub fn interpret_literal(
        &self,
        key_value: &HashMap<&str, String>,
        iter: &mut impl Iterator<Item = char>,
        out_str: &mut Vec<char>,
    ) {
        let mut literal = Vec::<char>::new();
        loop {
            let Some(ch) = iter.next() else {
                return;
            };

            match ch {
                ')' => break,
                _ => literal.push(ch),
            }
        }

        let literal_str: String = literal.into_iter().collect();

        let Some(value) = key_value.get(literal_str.as_str()) else {
            return;
        };
        for c in value.chars() {
            out_str.push(c)
        }
    }

    pub fn interpret_placeholder(
        &self,
        key_value: &HashMap<&str, String>,
        iter: &mut impl Iterator<Item = char>,
        out_str: &mut Vec<char>,
    ) {
        loop {
            let Some(ch) = iter.next() else {
                return;
            };

            match ch {
                '(' => {
                    self.interpret_literal(key_value, iter, out_str);
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
        loop {
            let Some(ch) = iter.next() else {
                break;
            };
            match ch {
                '%' => {
                    self.interpret_placeholder(key_value, &mut iter, &mut out_str);
                }
                _ => out_str.push(ch),
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
}
