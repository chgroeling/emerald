use super::resource_id_comps::ResourceIdComps;
use crate::error::{EmeraldError::*, Result};

#[derive(Debug, Clone, PartialEq, Hash, Default)]

/// A ResourceId points to a unique Resource
///
/// Currently a ResourceId is nothing else than a string containing a path
/// to the filesystem
pub struct ResourceId(pub String);

impl ResourceId {
    /// Splits a `ResourceId` into its components.
    pub fn split(&self) -> Result<ResourceIdComps> {
        let s = &self.0;
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
        let link = link_parts.last().ok_or(ValueError)?.to_string();

        let path: Option<String> = if link_parts.len() > 1 {
            Some(
                full_link[0..(full_link.len() - link_parts.last().ok_or(ValueError)?.len() - 1)]
                    .to_owned(),
            )
        } else {
            None
        };

        Ok(ResourceIdComps::new(link, path))
    }
}
// Allows to use a string as a ResourceId
impl From<&str> for ResourceId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for ResourceId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Eq for ResourceId {}

impl From<ResourceIdComps> for ResourceId {
    fn from(value: ResourceIdComps) -> Self {
        Self(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::ResourceId;

    #[test]
    fn test_simple_resource_id() {
        let test_rid: ResourceId = "[[test_res_id]]".into();
        let res = test_rid.split().unwrap();
        assert_eq!(res.name, "test_res_id");
    }

    #[test]
    fn test_resource_id_with_ext() {
        let test_rid: ResourceId = "[[test_res_id.md]]".into();
        let res = test_rid.split().unwrap();
        assert_eq!(res.name, "test_res_id.md");
    }

    #[test]
    fn test_none_path_from_simple_resource_id() {
        let test_rid: ResourceId = "[[test_res_id]]".into();
        let res = test_rid.split().unwrap();
        assert!(res.has_path() == false);
    }

    #[test]
    fn test_resource_id_with_path_1() {
        let test_rid: ResourceId = "[[a/b/c/test_res_id]]".into();
        let res = test_rid.split().unwrap();
        assert_eq!(res.name, "test_res_id");
    }

    #[test]
    fn test_resource_id_with_path_2() {
        let test_rid: ResourceId = "[[a/b/c/test_res_id]]".into();
        let res = test_rid.split().unwrap();
        let path = res.path.unwrap();
        assert_eq!(path, "a/b/c");
    }

    #[test]
    fn test_illegal_resource_id_front_space() {
        let test_rid: ResourceId = " [[test_res_id]]".into();
        let res = test_rid.split();
        assert!(res.is_err());
    }

    #[test]
    fn test_illegal_resource_id_tail_space() {
        let test_rid: ResourceId = "[[test_res_id]] ".into();
        let res = test_rid.split();
        assert!(res.is_err());
    }

    #[test]
    fn test_resource_id_with_leading_undescore() {
        let test_rid: ResourceId = "[[_test_res_id]]".into();
        let res = test_rid.split().unwrap();
        assert_eq!(res.name, "_test_res_id");
    }
}
