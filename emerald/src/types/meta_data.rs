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
    pub fn new_empty_stem(file_type: FileType) -> Self {
        MetaData {
            file_stem: "".into(),
            file_type,
        }
    }
}
