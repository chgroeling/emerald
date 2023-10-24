use super::md_provider::MdProvider;
use crate::error::EmeraldError::*;
use crate::error::Result;
use crate::{resources, types};

pub struct ContentMdProvider<T, U>
where
    T: resources::MdContentRetriever,
    U: resources::MetaDataLoader,
{
    content_loader: T,
    meta_data_loader: U,
}

impl<I, U> ContentMdProvider<I, U>
where
    I: resources::MdContentRetriever,
    U: resources::MetaDataLoader,
{
    pub fn new(content_loader: I, meta_data_loader: U) -> Self {
        Self {
            content_loader,
            meta_data_loader,
        }
    }
}
impl<I, U> MdProvider for ContentMdProvider<I, U>
where
    I: resources::MdContentRetriever,
    U: resources::MetaDataLoader,
{
    fn get_markdown(&self, rid: &types::ResourceId) -> Result<String> {
        let meta_data = self.meta_data_loader.load(rid)?;

        // do not allow anything other than markdown files pass this point
        let types::FileType::Markdown(_) = meta_data.file_type else {
            return Err(NotAMarkdownFile);
        };

        let res = self.content_loader.retrieve(rid)?;
        Ok(res.into())
    }
}
