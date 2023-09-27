#[derive(Debug)]
pub struct ResourceIdComponents {
    pub path: Option<String>,
    pub link: String,
}

impl ResourceIdComponents {
    pub fn new(link: String, path: Option<String>) -> Self {
        ResourceIdComponents { path, link }
    }

    #[allow(dead_code)]
    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    pub fn new_without_path(link: String) -> Self {
        ResourceIdComponents { path: None, link }
    }

    #[allow(dead_code)]
    pub fn new_with_path(name: String, path: String) -> Self {
        ResourceIdComponents {
            path: Some(path),
            link: name,
        }
    }
}
