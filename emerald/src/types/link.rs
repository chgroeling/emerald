use super::res_and_err::Result;
use super::{link_comps::LinkComps, split_link::SplitLink};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Link(pub String);

impl Link {
    pub fn split(&self) -> Result<LinkComps> {
        let split_link = SplitLink::new();
        split_link.split(self)
    }
}

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

impl From<LinkComps> for Link {
    fn from(value: LinkComps) -> Self {
        Self(value.to_string())
    }
}

impl Eq for Link {}
