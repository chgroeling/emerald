use std::fs;
use std::rc::Rc;

use crate::resources::endpoint_resolver::EndPointResolver;
use crate::types::Content;
use crate::types::EndPoint;
use crate::types::ResourceId;
use crate::Result;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::content_loader::ContentLoader;

pub struct FileContentLoader<I>
where
    I: EndPointResolver,
{
    ep_retriever: Rc<I>,
}

impl<I> FileContentLoader<I>
where
    I: EndPointResolver,
{
    pub fn new(ep_retriever: Rc<I>) -> Self {
        Self { ep_retriever }
    }
}

impl<I> ContentLoader for FileContentLoader<I>
where
    I: EndPointResolver,
{
    fn load(&self, resource_id: &ResourceId) -> Result<Content> {
        let endpoint = self.ep_retriever.resolve(resource_id)?;

        match endpoint {
            EndPoint::FileMarkdown(md_path) => Ok(fs::read_to_string(md_path)?.into()),
            EndPoint::FileUnknown(path) => Ok(fs::read_to_string(path)?.into()),
        }
    }
}

#[cfg(test)]
mod tests {}
