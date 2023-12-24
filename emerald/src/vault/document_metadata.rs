#[derive(Debug, Clone, PartialEq, Hash, Default)]
pub struct DocumentMetadata {
    pub tags: Option<String>,
    pub aliases: Vec<String>,
    pub keywords: Vec<String>,
    pub created: Option<String>,
    pub modified: Option<String>,
}
