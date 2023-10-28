use std::rc::Rc;

use super::md_provider::MdProvider;
use crate::error::EmeraldError::*;
use crate::error::Result;
use crate::model::note_model;
use crate::{resources, types};

pub struct ContentMdProvider<T>
where
    T: resources::MdContentRetriever,
{
    content_loader: T,
    meta_data_retriever: Rc<dyn note_model::MetaDataRetriever>,
}

impl<I> ContentMdProvider<I>
where
    I: resources::MdContentRetriever,
{
    pub fn new(
        content_loader: I,
        meta_data_retriever: Rc<dyn note_model::MetaDataRetriever>,
    ) -> Self {
        Self {
            content_loader,
            meta_data_retriever,
        }
    }
}
impl<I> MdProvider for ContentMdProvider<I>
where
    I: resources::MdContentRetriever,
{
    fn get_markdown(&self, rid: &types::ResourceId) -> Result<String> {
        let meta_data = self.meta_data_retriever.retrieve(rid);

        // do not allow anything other than markdown files pass this point
        let types::FileType::Markdown(_) = meta_data.file_type else {
            return Err(NotAMarkdownFile);
        };

        let res = self.content_loader.retrieve(rid)?;
        Ok(res.into())
    }
}
