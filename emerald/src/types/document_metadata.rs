use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub tags: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub created: Option<String>,
    pub modified: Option<String>,
    pub keywords: Option<Vec<String>>,
}

impl Default for DocumentMetadata {
    fn default() -> Self {
        Self {
            tags: Default::default(),
            aliases: Default::default(),
            created: Default::default(),
            modified: Default::default(),
            keywords: Default::default(),
        }
    }
}
