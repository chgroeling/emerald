use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DocumentMetadata {
    pub uid: Option<String>,
    pub tags: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub created: Option<String>,
    pub modified: Option<String>,
    pub keywords: Option<Vec<String>>,
}
