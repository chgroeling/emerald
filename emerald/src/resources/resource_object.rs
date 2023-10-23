use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum ResourceObject {
    File(PathBuf),
}

impl Eq for ResourceObject {}
