use super::{
    link_components::LinkComponents,
    res_and_err::{EmeraldError, Result},
};
use std::fmt::Display;

use EmeraldError::*;

impl Display for LinkComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(path_uw) = &self.path {
            write!(f, "[[{}/{}]]", path_uw, self.link)
        } else {
            write!(f, "[[{}]]", self.link)
        }
    }
}

impl From<&'static str> for LinkComponents {
    fn from(value: &'static str) -> Self {
        Self::new_link(value.to_owned())
    }
}

pub struct LinkDecomposer {}

impl LinkDecomposer {
    pub fn new() -> LinkDecomposer {
        LinkDecomposer {}
    }

    #[inline]
    fn extract_part<'a>(&self, s: &'a str) -> (&'a str, Option<&'a str>) {
        let end_idx = s.find(|c| c == '|' || c == '#' || c == '^');
        if end_idx.is_none() {
            return (s, None);
        }

        let end_idx = end_idx.map_or_else(|| s.len(), |x| x);

        let front = &s[..end_idx];
        let back = &s[end_idx..];

        (front, Some(back))
    }

    /// Splits a Wikilink stored in `s` into its parts and return as a DecomposedLink struct.
    pub fn decompose(&self, s: &str) -> Result<LinkComponents> {
        let start = s.find("[[").ok_or(NotAWikiLink)?;
        let end = s.find("]]").ok_or(NotAWikiLink)?;

        // check if "[[" is at the start of the string
        if start != 0 {
            return Err(NotAWikiLink);
        }

        // check if "]]" is at the end of the string
        if end != s.len() - 2 {
            return Err(NotAWikiLink);
        }

        // sanity check
        if start >= end {
            return Err(NotAWikiLink);
        }

        // the link text is inbetween the braces
        let link_text = &s[(start + 2)..end];

        // split string in half at the position of the first occurence of #^|
        let (mut front, mut back) = self.extract_part(link_text);

        // Get the full link and path if exists
        let full_link = front;
        let link_parts: Vec<&str> = full_link.split('/').collect();
        let link = link_parts.last().unwrap().to_string();

        let path: Option<String> = if link_parts.len() > 1 {
            Some(full_link[0..(full_link.len() - link_parts.last().unwrap().len() - 1)].to_owned())
        } else {
            None
        };

        let mut label: Option<String> = None;
        let mut section: Option<String> = None;
        let mut anchor: Option<String> = None;

        // Extract the rest
        while let Some(rest) = back {
            let first_char = &rest[0..1];

            // do look for the next occurance of #^| if any
            (front, back) = self.extract_part(&rest[1..]);

            match first_char {
                "|" => label = Some(front.to_owned()),
                "#" => section = Some(front.to_owned()),
                "^" => anchor = Some(front.to_owned()),
                _ => (),
            }
        }

        Ok(LinkComponents::new(link, path, label, section, anchor))
    }
}

#[cfg(test)]
mod link_decomposer_tests {
    use super::LinkDecomposer;

    #[test]
    fn test_simple_link() {
        let test_str = "[[test_link]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.link == "test_link"));
    }

    #[test]
    fn test_simple_link_with_ext() {
        let test_str = "[[test_link.md]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.link == "test_link.md"));
    }

    #[test]
    fn test_no_path_from_simple_link() {
        let test_str = "[[test_link]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.has_path() == false));
    }

    #[test]
    fn test_link_out_off_simple_link_with_name() {
        let test_str = "[[test_link|link_name]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.link == "test_link"));
    }

    #[test]
    fn test_link_out_off_link_with_path() {
        let test_str = "[[a/b/c/test_link]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.link == "test_link"));
    }

    #[test]
    fn test_link_out_off_link_with_path_and_section_link() {
        let test_str = "[[a/b/c/test_link#section_link]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.link == "test_link"));
    }

    #[test]
    fn test_link_out_off_link_with_path_and_section_link_and_name() {
        let test_str = "[[a/b/c/test_link#section_link|link_name]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.link == "test_link"));
    }

    #[test]
    fn test_path_out_off_link_with_short_path_and_section_link_and_name() {
        let test_str = "[[abc/test_link#section_link|link_name]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.path.is_some_and(|path| path == "abc")));
    }

    #[test]
    fn test_path_out_off_link_with_long_path_and_section_link_and_name() {
        let test_str = "[[a/b/c/test_link#section_link|link_name]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.path.is_some_and(|path| path == "a/b/c")));
    }

    #[test]
    fn test_path_out_off_link_with_long_absolute_path_and_section_link_and_name() {
        let test_str = "[[/a/b/c/test_link#section_link|link_name]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.path.is_some_and(|path| path == "/a/b/c")));
    }

    #[test]
    fn test_illegal_link_handling_front_space() {
        let test_str = " [[test_link]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_err());
    }

    #[test]
    fn test_illegal_link_handling_tail_space() {
        let test_str = "[[test_link]] ";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_err());
    }

    #[test]
    fn test_section_first_than_label_check_label() {
        let test_str = "[[test_link#section|label]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);
        let decomposed_link = res.unwrap();
        let label = decomposed_link.label.unwrap();
        assert_eq!(label, "label");
    }

    #[test]
    fn test_anchor_first_than_section_than_label_check_section() {
        let test_str = "[[test_link^anchor#section|label]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);
        let decomposed_link = res.unwrap();
        let section = decomposed_link.section.unwrap();
        assert_eq!(section, "section");
    }

    #[test]
    fn test_label_with_length0() {
        let test_str = "[[test_link|]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);
        let decomposed_link = res.unwrap();
        let section = decomposed_link.label.unwrap();
        assert_eq!(section, "");
    }
}
