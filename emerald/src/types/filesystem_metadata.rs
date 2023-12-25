use std::path::PathBuf;

use super::resource_type::ResourceType;

#[derive(Debug, Clone, PartialEq, Hash, Default)]
pub struct FilesystemMetadata {
    pub name: String,
    pub location: String,
    pub resource_type: ResourceType,
    pub size: u64,
    pub modified: i64,
    pub created: i64,
    pub path: PathBuf,
}

pub struct FilesystemMetadataBuilder {
    prep: FilesystemMetadata,
}

impl FilesystemMetadataBuilder {
    pub fn new() -> Self {
        Self {
            prep: Default::default(),
        }
    }

    pub fn set_name(self, name: String) -> Self {
        let new_prep = FilesystemMetadata { name, ..self.prep };
        Self { prep: new_prep }
    }

    pub fn set_location(self, location: String) -> Self {
        let new_prep = FilesystemMetadata {
            location,
            ..self.prep
        };
        Self { prep: new_prep }
    }

    pub fn set_resource_type(self, resource_type: ResourceType) -> Self {
        let new_prep = FilesystemMetadata {
            resource_type,
            ..self.prep
        };
        Self { prep: new_prep }
    }

    pub fn set_size(self, size: u64) -> Self {
        let new_prep = FilesystemMetadata { size, ..self.prep };
        Self { prep: new_prep }
    }

    pub fn set_modified(self, modified: i64) -> Self {
        let new_prep = FilesystemMetadata {
            modified,
            ..self.prep
        };
        Self { prep: new_prep }
    }

    pub fn set_created(self, created: i64) -> Self {
        let new_prep = FilesystemMetadata {
            created,
            ..self.prep
        };
        Self { prep: new_prep }
    }
    pub fn set_path(self, path: PathBuf) -> Self {
        let new_prep = FilesystemMetadata { path, ..self.prep };
        Self { prep: new_prep }
    }

    pub fn build(self) -> FilesystemMetadata {
        self.prep
    }
}
