#[derive(Debug, Clone, PartialEq, Hash)]
pub enum FileType {
    Unknown(String),
    Markdown(String),
    NoFileType(), // No file type available
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MetaData {
    pub file_stem: String,
    pub file_type: FileType,
}

impl MetaData {
    // builder pattern could work wonders here
}
