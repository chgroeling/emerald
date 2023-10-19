use crate::resources::endpoint_retriever::EndpointRetriever;
use crate::types::Content;
use crate::types::EndPoint;
use crate::types::ResourceId;
use crate::Result;
use std::fs;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::content_loader::ContentLoader;

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
    fn load(&self, resource_id: &ResourceId) -> Result<Content> {
        let endpoint = self.ep_retriever.retrieve(resource_id)?;

        match endpoint {
            EndPoint::FileMarkdown(md_path) => Ok(fs::read_to_string(md_path)?.into()),
            EndPoint::FileUnknown(path) => Ok(fs::read_to_string(path)?.into()),
        }
    }
}

#[cfg(test)]
mod tests {}
