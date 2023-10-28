use super::providers::{MdProvider, TitleProvider};
use crate::error::Result;
use crate::types;

pub struct Note {
    rid: types::ResourceId,
    title_provider: Box<dyn TitleProvider>,
    md_provider: Box<dyn MdProvider>,
}

impl Note {
    pub fn new(
        rid: types::ResourceId,
        title_provider: Box<dyn TitleProvider>,
        md_provider: Box<dyn MdProvider>,
    ) -> Self {
        Self {
            rid,
            title_provider,
            md_provider,
        }
    }

    pub fn title(&self) -> Result<String> {
        self.title_provider.get_title(&self.rid)
    }

    pub fn markdown(&self) -> String {
        self.md_provider.get_markdown(&self.rid)
    }
}
