use super::link_comps::LinkComps;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Link(pub String);

impl Link {}

// ALlows to use a string as a link
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

impl From<&LinkComps> for Link {
    fn from(value: &LinkComps) -> Self {
        Self(value.to_string())
    }
}

impl Eq for Link {}
