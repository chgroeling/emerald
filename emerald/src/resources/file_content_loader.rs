use super::content_loader::ContentLoader;
use super::resource_object::ResourceObject;
use super::resource_object_retriever::ResourceObjectRetriever;
use crate::error::Result;
use crate::types;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::fs;

#[derive(Clone)]
pub struct FileContentLoader<I>
where
    I: ResourceObjectRetriever,
{
    ro_retriever: I,
}

impl<I> FileContentLoader<I>
where
    I: ResourceObjectRetriever,
{
    pub fn new(ro_retriever: I) -> Self {
        Self { ro_retriever }
    }
}

impl<I> ContentLoader for FileContentLoader<I>
where
    I: ResourceObjectRetriever,
{
    fn load(&self, rid: &types::ResourceId) -> Result<types::Content> {
        let ro = self.ro_retriever.retrieve(rid)?;

        match ro {
            ResourceObject::File(md_path) => Ok(fs::read_to_string(md_path)?.into()),
        }
    }
}
