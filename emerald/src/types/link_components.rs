#[derive(Debug)]
pub struct DecomposedLink {
    pub path: Option<String>,
    pub link: String,
    pub label: Option<String>,
    pub section: Option<String>,
    pub anchor: Option<String>,
}
