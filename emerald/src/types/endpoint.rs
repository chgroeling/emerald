use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum EndPoint {
    FileUnknown(PathBuf),
    FileMarkdown(PathBuf),
}
