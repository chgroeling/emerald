use super::resource_type::ResourceType;

#[derive(Debug, Clone, PartialEq, Hash, Default)]
pub struct MetaData {
    pub name: String,
    pub resource_type: ResourceType,
    pub size: u64,
    pub modified: i64,
    pub created: i64,
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

    pub fn set_name(self, name: String) -> Self {
        let new_prep = MetaData { name, ..self.prep };
        Self { prep: new_prep }
    }

    pub fn set_resource_type(self, resource_type: ResourceType) -> Self {
        let new_prep = MetaData {
            resource_type,
            ..self.prep
        };
        Self { prep: new_prep }
    }

    pub fn set_size(self, size: u64) -> Self {
        let new_prep = MetaData { size, ..self.prep };
        Self { prep: new_prep }
    }

    pub fn set_modified(self, modified: i64) -> Self {
        let new_prep = MetaData {
            modified,
            ..self.prep
        };
        Self { prep: new_prep }
    }

    pub fn set_created(self, created: i64) -> Self {
        let new_prep = MetaData {
            created,
            ..self.prep
        };
        Self { prep: new_prep }
    }

    pub fn build(self) -> MetaData {
        self.prep
    }
}
