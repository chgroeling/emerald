use std::fs::File;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum FileType {
    Unknown(String),
    Markdown(String),
    NoFileType(), // No file type available
}

impl Default for FileType {
    fn default() -> Self {
        Self::NoFileType()
    }
}
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MetaData {
    pub file_stem: String,
    pub file_type: FileType,
    pub modified: u64,
    pub created: u64,
}

impl Default for MetaData {
    fn default() -> Self {
        Self {
            file_stem: Default::default(),
            file_type: Default::default(),
            modified: Default::default(),
            created: Default::default(),
        }
    }
}
pub struct MetaDataBuilder {
    prep: MetaData,
}

impl MetaDataBuilder {
    pub fn new() -> Self {
        Self {
            prep: Default::default(),
        }
    }

    pub fn set_file_stem(self, stem: String) -> Self {
        let new_prep = MetaData {
            file_stem: stem,
            ..self.prep
        };
        Self { prep: new_prep }
    }

    pub fn set_file_type(self, file_type: FileType) -> Self {
        let new_prep = MetaData {
            file_type,
            ..self.prep
        };
        Self { prep: new_prep }
    }

    pub fn set_modified(self, modified: u64) -> Self {
        let new_prep = MetaData {
            modified: modified,
            ..self.prep
        };
        Self { prep: new_prep }
    }

    pub fn set_created(self, created: u64) -> Self {
        let new_prep = MetaData {
            created: created,
            ..self.prep
        };
        Self { prep: new_prep }
    }

    pub fn build(self) -> MetaData {
        self.prep
    }
}
