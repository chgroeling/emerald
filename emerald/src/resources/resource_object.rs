use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum ResourceObject {
    FileUnknown(PathBuf),
    FileMarkdown(PathBuf),
}

impl Eq for ResourceObject {}
