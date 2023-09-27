use super::{
    res_and_err::{EmeraldError, Result},
    resource_id_comps::ResourceIdComps,
    ResourceId,
};
use EmeraldError::*;

pub struct SplitResourceId {}

impl SplitResourceId {
    pub fn new() -> SplitResourceId {
        SplitResourceId {}
    }

    /// Splits a ResourceId stored in `s` into its parts and return as a ResourceIdComponents struct.
    pub fn split(&self, res_id: &ResourceId) -> Result<ResourceIdComps> {
        let s = &res_id.0;
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

        Ok(ResourceIdComps::new(link, path))
    }
}

#[cfg(test)]
mod tests {
    use super::SplitResourceId;

    #[test]
    fn test_simple_resource_id() {
        let test_str = "[[test_res_id]]";
        let dut = SplitResourceId::new();

        let res = dut.split(&test_str.into()).unwrap();

        assert_eq!(res.name, "test_res_id");
    }

    #[test]
    fn test_resource_id_with_ext() {
        let test_str = "[[test_res_id.md]]";
        let dut = SplitResourceId::new();

        let res = dut.split(&test_str.into()).unwrap();

        assert_eq!(res.name, "test_res_id.md");
    }

    #[test]
    fn test_none_path_from_simple_resource_id() {
        let test_str = "[[test_res_id]]";
        let dut = SplitResourceId::new();

        let res = dut.split(&test_str.into()).unwrap();

        assert!(res.has_path() == false);
    }

    #[test]
    fn test_resource_id_with_path_1() {
        let test_str = "[[a/b/c/test_res_id]]";
        let dut = SplitResourceId::new();

        let res = dut.split(&test_str.into()).unwrap();

        assert_eq!(res.name, "test_res_id");
    }

    #[test]
    fn test_resource_id_with_path_2() {
        let test_str = "[[a/b/c/test_res_id]]";
        let dut = SplitResourceId::new();

        let res = dut.split(&test_str.into()).unwrap();
        let path = res.path.unwrap();
        assert_eq!(path, "a/b/c");
    }

    #[test]
    fn test_illegal_resource_id_front_space() {
        let test_str = " [[test_res_id]]";
        let dut = SplitResourceId::new();

        let res = dut.split(&test_str.into());
        assert!(res.is_err());
    }

    #[test]
    fn test_illegal_resource_id_tail_space() {
        let test_str = "[[test_res_id]] ";
        let dut = SplitResourceId::new();

        let res = dut.split(&test_str.into());
        assert!(res.is_err());
    }

    #[test]
    fn test_resource_id_with_leading_undescore() {
        let test_str = "[[_test_res_id]]";
        let dut = SplitResourceId::new();

        let res = dut.split(&test_str.into());
        let link_components = res.unwrap();
        assert_eq!(link_components.name, "_test_res_id");
    }
}
