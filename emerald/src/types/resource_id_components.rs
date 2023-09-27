#[derive(Debug)]
pub struct ResourceIdComponents {
    pub path: Option<String>,
    pub link: String,
    pub label: Option<String>,
    pub section: Option<String>,
    pub anchor: Option<String>,
}

impl ResourceIdComponents {
    pub fn new(
        link: String,
        path: Option<String>,
        label: Option<String>,
        section: Option<String>,
        anchor: Option<String>,
    ) -> Self {
        ResourceIdComponents {
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
        ResourceIdComponents {
            path: None,
            link,
            label: None,
            section: None,
            anchor: None,
        }
    }

    #[allow(dead_code)]
    pub fn new_link_with_path(name: String, path: String) -> Self {
        ResourceIdComponents {
            path: Some(path),
            link: name,
            label: None,
            section: None,
            anchor: None,
        }
    }
}
