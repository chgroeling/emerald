use super::res_and_err::{EmeraldError, Result};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
pub struct DecomposedLink {
    pub path: Option<String>,
    pub name: String,
}

impl Display for DecomposedLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(path_uw) = &self.path {
            write!(f, "[[{}/{}]]", path_uw, self.name)
        } else {
            write!(f, "[[{}]]", self.name)
        }
    }
}
impl DecomposedLink {
    pub fn new(link_name: String, link_path: Option<String>) -> Self {
        DecomposedLink {
            path: link_path,
            name: link_name,
        }
    }

    #[allow(dead_code)]
    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    pub fn new_link(link: String) -> Self {
        DecomposedLink {
            path: None,
            name: link,
        }
    }

    #[allow(dead_code)]
    pub fn new_link_with_path(name: String, path: String) -> Self {
        DecomposedLink {
            path: Some(path),
            name,
        }
    }
}

impl From<&'static str> for DecomposedLink {
    fn from(value: &'static str) -> Self {
        Self::new_link(value.to_owned())
    }
}

pub struct LinkDecomposer {}

fn extract_wiki_link(s: &str) -> Option<HashMap<String, String>> {
    let start = s.find("[[")?;
    let end = s.find("]]")?;

    if start != 0 {
        return None;
    }

    if end != s.len() - 2 {
        return None;
    }
    if start < end {
        let link_text = &s[(start + 2)..end];
        let parts: Vec<&str> = link_text
            .split(|c| c == '|' || c == '#' || c == '^')
            .collect();
        let mut map: HashMap<String, String> = HashMap::new();

        // Get the full link and path if exists
        let full_link = parts[0];
        let link_parts: Vec<&str> = full_link.split('/').collect();
        map.insert("link".to_owned(), link_parts.last().unwrap().to_string());
        if link_parts.len() > 1 {
            map.insert(
                "path".to_owned(),
                full_link[0..(full_link.len() - link_parts.last().unwrap().len() - 1)].to_owned(),
            );
        }

        if parts.len() > 1 {
            map.insert("label".to_owned(), parts[1].to_owned());
        }
        if parts.len() > 2 {
            map.insert("section".to_owned(), parts[2].to_owned());
        }
        if parts.len() > 3 {
            map.insert("anchor".to_owned(), parts[3].to_owned());
        }

        Some(map)
    } else {
        None
    }
}

impl LinkDecomposer {
    pub fn new() -> LinkDecomposer {
        LinkDecomposer {}
    }

    pub fn decompose(&self, link: &str) -> Result<DecomposedLink> {
        let hashmap = extract_wiki_link(link).ok_or(EmeraldError::NotAWikiLink)?;

        let extracted_link = hashmap
            .get("link")
            .ok_or(EmeraldError::NotAWikiLink)?
            .to_owned();
        let extracted_path = hashmap.get("path").map(|f| f.to_owned());

        Ok(DecomposedLink::new(
            extracted_link,
            extracted_path.to_owned(),
        ))
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

        assert!(res.is_ok_and(|link| link.name == "test_link"));
    }

    #[test]
    fn test_simple_link_with_ext() {
        let test_str = "[[test_link.md]]";
        let ldec = LinkDecomposer::new();

        let res = ldec.decompose(test_str);

        assert!(res.is_ok_and(|link| link.name == "test_link.md"));
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
