use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DocumentMetaData {
    tags: Option<String>,
    aliases: Option<Vec<String>>,
    created: Option<String>,
    modified: Option<String>,
    keywords: Option<Vec<String>>,
}
