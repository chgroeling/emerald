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
        if let Some(path_uw) = &self.path {
            write!(f, "[[{}/{}]]", path_uw, self.name)
        } else {
            write!(f, "[[{}]]", self.name)
        }
    }
}

impl From<&'static str> for ResourceIdComps {
    fn from(value: &'static str) -> Self {
        Self::new_without_path(value.to_owned())
    }
}
