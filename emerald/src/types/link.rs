use super::res_and_err::Result;
use super::{link_comps::LinkComps, EmeraldError};
use EmeraldError::*;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Link(pub String);

impl Link {
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

    /// Splits a `Link` into its components.
    pub fn split(&self) -> Result<LinkComps> {
        let s = &self.0;
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
        let link = link_parts.last().ok_or(ValueError)?.to_string();

        let path: Option<String> = if link_parts.len() > 1 {
            Some(
                full_link[0..(full_link.len() - link_parts.last().ok_or(ValueError)?.len() - 1)]
                    .to_owned(),
            )
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

        Ok(LinkComps::new(link, path, label, section, anchor))
    }
}

// ALlows to use a string as a link
impl From<&str> for Link {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
impl From<String> for Link {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<LinkComps> for Link {
    fn from(value: LinkComps) -> Self {
        Self(value.to_string())
    }
}

impl Eq for Link {}

#[cfg(test)]
mod tests {
    use super::Link;

    #[test]
    fn test_simple_link() {
        let test_link: Link = "[[test_link]]".into();

        let res = test_link.split();

        assert!(res.is_ok_and(|link| link.name == "test_link"));
    }

    #[test]
    fn test_simple_link_with_ext() {
        let test_link: Link = "[[test_link.md]]".into();

        let res = test_link.split();

        assert!(res.is_ok_and(|link| link.name == "test_link.md"));
    }

    #[test]
    fn test_no_path_from_simple_link() {
        let test_link: Link = "[[test_link]]".into();

        let res = test_link.split();

        assert!(res.is_ok_and(|link| link.has_path() == false));
    }

    #[test]
    fn test_link_out_off_simple_link_with_name() {
        let test_link: Link = "[[test_link|link_name]]".into();

        let res = test_link.split();

        assert!(res.is_ok_and(|link| link.name == "test_link"));
    }

    #[test]
    fn test_link_out_off_link_with_path() {
        let test_link: Link = "[[a/b/c/test_link]]".into();

        let res = test_link.split();

        assert!(res.is_ok_and(|link| link.name == "test_link"));
    }

    #[test]
    fn test_link_out_off_link_with_path_and_section_link() {
        let test_link: Link = "[[a/b/c/test_link#section_link]]".into();

        let res = test_link.split();

        assert!(res.is_ok_and(|link| link.name == "test_link"));
    }

    #[test]
    fn test_link_out_off_link_with_path_and_section_link_and_name() {
        let test_link: Link = "[[a/b/c/test_link#section_link|link_name]]".into();

        let res = test_link.split();

        assert!(res.is_ok_and(|link| link.name == "test_link"));
    }

    #[test]
    fn test_path_out_off_link_with_short_path_and_section_link_and_name() {
        let test_link: Link = "[[abc/test_link#section_link|link_name]]".into();

        let res = test_link.split();

        assert!(res.is_ok_and(|link| link.path.is_some_and(|path| path == "abc")));
    }

    #[test]
    fn test_path_out_off_link_with_long_path_and_section_link_and_name() {
        let test_link: Link = "[[a/b/c/test_link#section_link|link_name]]".into();

        let res = test_link.split();

        assert!(res.is_ok_and(|link| link.path.is_some_and(|path| path == "a/b/c")));
    }

    #[test]
    fn test_path_out_off_link_with_long_absolute_path_and_section_link_and_name() {
        let test_link: Link = "[[/a/b/c/test_link#section_link|link_name]]".into();
        let res = test_link.split().unwrap();

        let path = res.path.unwrap();
        assert_eq!(path, "/a/b/c");
    }

    #[test]
    fn test_illegal_link_handling_front_space() {
        let test_link: Link = " [[test_link]]".into();
        let res = test_link.split();

        assert!(res.is_err());
    }

    #[test]
    fn test_illegal_link_handling_tail_space() {
        let test_link: Link = "[[test_link]] ".into();
        let res = test_link.split();
        assert!(res.is_err());
    }

    #[test]
    fn test_section_first_than_label_check_label() {
        let test_link: Link = "[[test_link#section|label]]".into();
        let res = test_link.split();
        let link_components = res.unwrap();
        let label = link_components.label.unwrap();
        assert_eq!(label, "label");
    }

    #[test]
    fn test_anchor_first_than_section_than_label_check_section() {
        let test_link: Link = "[[test_link^anchor#section|label]]".into();
        let res = test_link.split();
        let link_components = res.unwrap();
        let section = link_components.section.unwrap();
        assert_eq!(section, "section");
    }

    #[test]
    fn test_label_with_length0() {
        let test_link: Link = "[[test_link|]]".into();
        let res = test_link.split();
        let link_components = res.unwrap();
        let section = link_components.label.unwrap();
        assert_eq!(section, "");
    }

    #[test]
    fn test_link_with_leading_undescore() {
        let test_link: Link = "[[_test_link]]".into();
        let res = test_link.split();
        let link_components = res.unwrap();
        assert!(link_components.name == "_test_link");
    }
}
