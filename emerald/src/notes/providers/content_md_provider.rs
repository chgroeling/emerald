use std::rc::Rc;

use super::md_provider::MdProvider;
use crate::error::EmeraldError::*;
use crate::error::Result;
use crate::model::content;
use crate::model::note;
use crate::types;

pub struct ContentMdProvider<T>
where
    T: content::MdContentRetriever,
{
    content_loader: T,
    meta_data_retriever: Rc<dyn note::MetaDataRetriever>,
}

impl<I> ContentMdProvider<I>
where
    I: content::MdContentRetriever,
{
    pub fn new(content_loader: I, meta_data_retriever: Rc<dyn note::MetaDataRetriever>) -> Self {
        Self {
            content_loader,
            meta_data_retriever,
        }
    }
}
impl<I> MdProvider for ContentMdProvider<I>
where
    I: content::MdContentRetriever,
{
    fn get_markdown(&self, rid: &types::ResourceId) -> Result<String> {
        let meta_data = self.meta_data_retriever.retrieve(rid);

        // do not allow anything other than markdown files pass this point
        let types::FileType::Markdown(_) = meta_data.file_type else {
            return Err(NotAMarkdownFile);
        };

        let res = self.content_loader.retrieve(rid);
        Ok(res.into())
    }
}
