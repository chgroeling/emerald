use super::content_loader::ContentLoader;
use super::endpoint_retriever::EndpointRetriever;
use crate::error::Result;
use crate::types;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::fs;

#[derive(Clone)]
pub struct FileContentLoader<I>
where
    I: EndpointRetriever,
{
    ep_retriever: I,
}

impl<I> FileContentLoader<I>
where
    I: EndpointRetriever,
{
    pub fn new(ep_retriever: I) -> Self {
        Self { ep_retriever }
    }
}

impl<I> ContentLoader for FileContentLoader<I>
where
    I: EndpointRetriever,
{
    fn load(&self, resource_id: &types::ResourceId) -> Result<types::Content> {
        let endpoint = self.ep_retriever.retrieve(resource_id)?;

        match endpoint {
            types::ResourceObject::FileMarkdown(md_path) => Ok(fs::read_to_string(md_path)?.into()),
            types::ResourceObject::FileUnknown(path) => Ok(fs::read_to_string(path)?.into()),
        }
    }
}
