use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum EndPoint {
    File(PathBuf),
    FileMarkdown(PathBuf),
}
