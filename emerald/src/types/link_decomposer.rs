use super::res_and_err::{EmeraldError, Result};
use std::fmt::Display;

#[derive(Debug)]
pub struct DecomposedLink {
    pub path: Option<String>,
    pub link: String,
    pub label: Option<String>,
    pub section: Option<String>,
    pub anchor: Option<String>,
}

impl Display for DecomposedLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(path_uw) = &self.path {
            write!(f, "[[{}/{}]]", path_uw, self.link)
        } else {
            write!(f, "[[{}]]", self.link)
        }
    }
}
impl DecomposedLink {
    pub fn new(
        link: String,
        path: Option<String>,
        label: Option<String>,
        section: Option<String>,
        anchor: Option<String>,
    ) -> Self {
        DecomposedLink {
            path,
            link,
            label,
            section,
            anchor,
        }
    }

    #[allow(dead_code)]
    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    pub fn new_link(link: String) -> Self {
        DecomposedLink {
            path: None,
            link,
            label: None,
            section: None,
            anchor: None,
        }
    }

    #[allow(dead_code)]
    pub fn new_link_with_path(name: String, path: String) -> Self {
        DecomposedLink {
            path: Some(path),
            link: name,
            label: None,
            section: None,
            anchor: None,
        }
    }
}

impl From<&'static str> for DecomposedLink {
    fn from(value: &'static str) -> Self {
        Self::new_link(value.to_owned())
    }
}

pub struct LinkDecomposer {}

fn extract_wiki_link(s: &str) -> Option<DecomposedLink> {
    let start = s.find("[[")?;
    let end = s.find("]]")?;

    // check if "[[" is at the start of the string
    if start != 0 {
        return None;
    }

    // check if "]]" is at the end of the string
    if end != s.len() - 2 {
        return None;
    }

    // sanity check
    if start >= end {
        return None;
    }

    let link_text = &s[(start + 2)..end];
    let parts: Vec<&str> = link_text
        .split(|c| c == '|' || c == '#' || c == '^')
        .collect();

    // Get the full link and path if exists
    let full_link = parts[0];
    let link_parts: Vec<&str> = full_link.split('/').collect();
    let link = link_parts.last().unwrap().to_string();

    let path: Option<String> = if link_parts.len() > 1 {
        Some(full_link[0..(full_link.len() - link_parts.last().unwrap().len() - 1)].to_owned())
    } else {
        None
    };

    let label: Option<String> = if parts.len() > 1 {
        Some(parts[1].to_owned())
    } else {
        None
    };
    let section: Option<String> = if parts.len() > 2 {
        Some(parts[2].to_owned())
    } else {
        None
    };

    let anchor: Option<String> = if parts.len() > 3 {
        Some(parts[3].to_owned())
    } else {
        None
    };

    Some(DecomposedLink::new(link, path, label, section, anchor))
}

impl LinkDecomposer {
    pub fn new() -> LinkDecomposer {
        LinkDecomposer {}
    }

    pub fn decompose(&self, link: &str) -> Result<DecomposedLink> {
        Ok(extract_wiki_link(link).ok_or(EmeraldError::NotAWikiLink)?)
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

        assert!(res.is_ok_and(|link| link.link == "test_link"));
    }

    #[test]
    fn link_out_off_link_with_path() {
        let test_str = "[[a/b/c/test_link]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.link == "test_link"));
    }

    #[test]
    fn link_out_off_link_with_path_and_section_link() {
        let test_str = "[[a/b/c/test_link#section_link]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.link == "test_link"));
    }

    #[test]
    fn link_out_off_link_with_path_and_section_link_and_name() {
        let test_str = "[[a/b/c/test_link#section_link|link_name]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.link == "test_link"));
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
