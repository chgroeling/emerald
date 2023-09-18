#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Link(pub String);

impl Link {}
impl From<&str> for Link {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
impl From<String> for Link {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Eq for Link {}
