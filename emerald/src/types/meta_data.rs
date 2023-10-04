#[derive(Debug, Clone, PartialEq, Hash)]
pub enum FileType {
    Unknown(String),
    Markdown(String),
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MetaData {
    pub file_stem: String,
    pub file_type: FileType,
}
