use std::fmt::Display;

#[derive(Debug)]
pub struct ResourceIdComps {
    pub path: Option<String>,
    pub name: String,
}

impl ResourceIdComps {
    pub fn new(link: String, path: Option<String>) -> Self {
        ResourceIdComps { path, name: link }
    }

    #[allow(dead_code)]
    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    pub fn new_without_path(name: String) -> Self {
        ResourceIdComps { path: None, name }
    }

    #[allow(dead_code)]
    pub fn new_with_path(name: String, path: String) -> Self {
        ResourceIdComps {
            path: Some(path),
            name,
        }
    }
}

impl Display for ResourceIdComps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[[")?;

        if let Some(path) = &self.path {
            write!(f, "{}/", path)?;
        }
        write!(f, "{}", self.name)?;

        write!(f, "]]")
    }
}

impl From<&'static str> for ResourceIdComps {
    fn from(value: &'static str) -> Self {
        Self::new_without_path(value.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::ResourceIdComps;

    #[test]
    fn test_fmt_bare_resource_id() {
        let dut = ResourceIdComps::new_without_path("my_resource_id".into());
        let res = dut.to_string();
        assert_eq!(res, "[[my_resource_id]]")
    }

    #[test]
    fn test_fmt_full_resource_id() {
        let dut = ResourceIdComps::new_with_path("my_resource_id".into(), "a/b/c".into());
        let res = dut.to_string();
        assert_eq!(res, "[[a/b/c/my_resource_id]]")
    }
}
