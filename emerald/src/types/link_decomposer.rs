use std::fmt::Display;

use regex::Regex;

use super::res_and_err::{EmeraldError, Result};

#[derive(Debug)]
pub struct DecomposedLink<'a> {
    pub path: Option<&'a str>,
    pub name: &'a str,
}

impl<'a> Display for DecomposedLink<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(path_uw) = &self.path {
            write!(f, "[[{}/{}]]", path_uw, self.name)
        } else {
            write!(f, "[[{}]]", self.name)
        }
    }
}
impl<'a> DecomposedLink<'a> {
    pub fn new(link_name: &'a str, link_path: Option<&'a str>) -> Self {
        DecomposedLink {
            path: link_path,
            name: link_name,
        }
    }

    #[allow(dead_code)]
    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    pub fn new_link(link: &'a str) -> Self {
        DecomposedLink {
            path: None,
            name: link,
        }
    }

    #[allow(dead_code)]
    pub fn new_link_with_path(name: &'a str, path: &'a str) -> Self {
        DecomposedLink {
            path: Some(path),
            name,
        }
    }
}

impl<'a> From<&'static str> for DecomposedLink<'a> {
    fn from(value: &'static str) -> Self {
        Self::new_link(value)
    }
}

pub struct LinkDecomposer {
    regex: Regex,
}

impl LinkDecomposer {
    pub fn new() -> LinkDecomposer {
        // TODO: INSTEAD OF USING REGEX WRITE OWN WIKI LINK PARSER. SHOULD BE FASTER
        let link_regex = r"^\[{2}(?:([^\]#|]*)[\/])?(.*?)([#|][^\]#|]*)?([#|][^\]#|]*)?\]{2}$";
        let re = Regex::new(link_regex).unwrap();

        LinkDecomposer { regex: re }
    }

    pub fn decompose<'a>(&self, link: &'a str) -> Result<DecomposedLink<'a>> {
        let res = self
            .regex
            .captures(link)
            .ok_or(EmeraldError::NotAWikiLink)?;

        let extracted_link = res.get(2).ok_or(EmeraldError::NotAWikiLink)?;
        let extracted_path = res.get(1).map(|path_match| path_match.as_str());

        Ok(DecomposedLink::new(extracted_link.as_str(), extracted_path))
    }
}

#[cfg(test)]
mod wiki_link_decomposer_tests {
    use super::LinkDecomposer;

    #[test]
    fn link_out_off_simple_link() {
        let test_str = "[[test_link]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.name == "test_link"));
    }

    #[test]
    fn check_no_path_from_simple_link() {
        let test_str = "[[test_link]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.has_path() == false));
    }

    #[test]
    fn link_out_off_simple_link_with_name() {
        let test_str = "[[test_link|link_name]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.name == "test_link"));
    }

    #[test]
    fn link_out_off_link_with_path() {
        let test_str = "[[a/b/c/test_link]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.name == "test_link"));
    }

    #[test]
    fn link_out_off_link_with_path_and_section_link() {
        let test_str = "[[a/b/c/test_link#section_link]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.name == "test_link"));
    }

    #[test]
    fn link_out_off_link_with_path_and_section_link_and_name() {
        let test_str = "[[a/b/c/test_link#section_link|link_name]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.name == "test_link"));
    }

    #[test]
    fn path_out_off_link_with_short_path_and_section_link_and_name() {
        let test_str = "[[abc/test_link#section_link|link_name]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.path.is_some_and(|path| path == "abc")));
    }

    #[test]
    fn path_out_off_link_with_long_path_and_section_link_and_name() {
        let test_str = "[[a/b/c/test_link#section_link|link_name]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.path.is_some_and(|path| path == "a/b/c")));
    }

    #[test]
    fn path_out_off_link_with_long_absolute_path_and_section_link_and_name() {
        let test_str = "[[/a/b/c/test_link#section_link|link_name]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.path.is_some_and(|path| path == "/a/b/c")));
    }

    #[test]
    fn illegal_link_handling_front_space() {
        let test_str = " [[test_link]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_err());
    }

    #[test]
    fn illegal_link_handling_tail_space() {
        let test_str = "[[test_link]] ";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_err());
    }
}
