use unicode_normalization::UnicodeNormalization;

#[inline]
pub fn normalize_str(inp: &str) -> String {
    normalize_str_iter(inp).collect()
}

#[inline]
pub fn normalize_str_iter(inp: &str) -> impl Iterator<Item = char> + '_ {
    inp.nfc().map(|ch| match ch {
        '\u{a0}' => ' ',
        _ => ch,
    })
}

#[cfg(test)]
mod tests {
    use super::normalize_str;

    #[test]
    fn check_identity() {
        let result = normalize_str("abc");
        assert_eq!(result, "abc");
    }

    #[test]
    fn check_trailing_spaces_with_nbsp() {
        let result = normalize_str("abc   ");
        assert_eq!(result, "abc   ");
    }

    #[test]
    fn check_nfc_conversion() {
        // NKD -> NFC representation of umlauts
        //     NKD         NFC
        //    ---------   -------
        // ä   0x61CC88 -> C3A4
        // ö   0x6FCC88 -> C3B6
        let result = normalize_str("öä"); // ö and ä in NFD
        assert_eq!(result, "öä");
    }

    #[test]
    fn check_non_breaking_space_conversion() {
        let result = normalize_str("hello world");
        assert_eq!(result, "hello world");
    }
}
