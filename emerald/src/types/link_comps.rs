use std::fmt::Display;

#[derive(Debug)]
pub struct LinkComps {
    pub path: Option<String>,
    pub name: String,
    pub label: Option<String>,
    pub section: Option<String>,
    pub anchor: Option<String>,
}

impl LinkComps {
    pub fn new(
        name: String,
        path: Option<String>,
        label: Option<String>,
        section: Option<String>,
        anchor: Option<String>,
    ) -> Self {
        LinkComps {
            path,
            name,
            label,
            section,
            anchor,
        }
    }

    #[allow(dead_code)]
    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    pub fn new_link(name: String) -> Self {
        LinkComps {
            path: None,
            name,
            label: None,
            section: None,
            anchor: None,
        }
    }

    #[allow(dead_code)]
    pub fn new_link_with_path(name: String, path: String) -> Self {
        LinkComps {
            path: Some(path),
            name,
            label: None,
            section: None,
            anchor: None,
        }
    }
}

impl Display for LinkComps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(path_uw) = &self.path {
            write!(f, "[[{}/{}]]", path_uw, self.name)
        } else {
            write!(f, "[[{}]]", self.name)
        }
    }
}

impl From<&'static str> for LinkComps {
    fn from(value: &'static str) -> Self {
        Self::new_link(value.to_owned())
    }
}
