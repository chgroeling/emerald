use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DocumentMetadata {
    tags: Option<String>,
    aliases: Option<Vec<String>>,
    created: Option<String>,
    modified: Option<String>,
    keywords: Option<Vec<String>>,
}
