use super::{
    res_and_err::{EmeraldError, Result},
    resource_id_components::ResourceIdComponents,
};
use std::fmt::Display;

use EmeraldError::*;

impl Display for ResourceIdComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(path_uw) = &self.path {
            write!(f, "[[{}/{}]]", path_uw, self.link)
        } else {
            write!(f, "[[{}]]", self.link)
        }
    }
}

impl From<&'static str> for ResourceIdComponents {
    fn from(value: &'static str) -> Self {
        Self::new_without_path(value.to_owned())
    }
}

pub struct SplitResourceId {}

impl SplitResourceId {
    pub fn new() -> SplitResourceId {
        SplitResourceId {}
    }

    /// Splits a ResourceId stored in `s` into its parts and return as a ResourceIdComponents struct.
    pub fn split(&self, s: &str) -> Result<ResourceIdComponents> {
        let start = s.find("[[").ok_or(NotAResourceId)?;
        let end = s.find("]]").ok_or(NotAResourceId)?;

        // check if "[[" is at the start of the string
        if start != 0 {
            return Err(NotAResourceId);
        }

        // check if "]]" is at the end of the string
        if end != s.len() - 2 {
            return Err(NotAResourceId);
        }

        // sanity check
        if start >= end {
            return Err(NotAResourceId);
        }

        // the link text is inbetween the braces
        let link_text = &s[(start + 2)..end];

        // Get the full link and path if exists
        let full_link = link_text;
        let link_parts: Vec<&str> = full_link.split('/').collect();
        let link = link_parts.last().unwrap().to_string();

        let path: Option<String> = if link_parts.len() > 1 {
            Some(full_link[0..(full_link.len() - link_parts.last().unwrap().len() - 1)].to_owned())
        } else {
            None
        };

        Ok(ResourceIdComponents::new(link, path))
    }
}

#[cfg(test)]
mod tests {
    use super::SplitResourceId;

    #[test]
    fn test_simple_link() {
        let test_str = "[[test_link]]";
        let dut = SplitResourceId::new();

        let res = dut.split(test_str);

        assert!(res.is_ok_and(|link| link.link == "test_link"));
    }

    #[test]
    fn test_simple_link_with_ext() {
        let test_str = "[[test_link.md]]";
        let dut = SplitResourceId::new();

        let res = dut.split(test_str);

        assert!(res.is_ok_and(|link| link.link == "test_link.md"));
    }

    #[test]
    fn test_no_path_from_simple_link() {
        let test_str = "[[test_link]]";
        let dut = SplitResourceId::new();

        let res = dut.split(test_str);

        assert!(res.is_ok_and(|link| link.has_path() == false));
    }

    #[test]
    fn test_link_out_off_link_with_path() {
        let test_str = "[[a/b/c/test_link]]";
        let dut = SplitResourceId::new();

        let res = dut.split(test_str);

        assert!(res.is_ok_and(|link| link.link == "test_link"));
    }

    #[test]
    fn test_illegal_link_handling_front_space() {
        let test_str = " [[test_link]]";
        let dut = SplitResourceId::new();

        let res = dut.split(test_str);
        assert!(res.is_err());
    }

    #[test]
    fn test_illegal_link_handling_tail_space() {
        let test_str = "[[test_link]] ";
        let dut = SplitResourceId::new();

        let res = dut.split(test_str);
        assert!(res.is_err());
    }

    #[test]
    fn test_link_with_leading_undescore() {
        let test_str = "[[_test_link]]";
        let dut = SplitResourceId::new();

        let res = dut.split(test_str);
        let link_components = res.unwrap();
        assert!(link_components.link == "_test_link");
    }
}
