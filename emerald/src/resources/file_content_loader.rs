use std::fs;
use std::rc::Rc;

use crate::maps::resource_id_retriever::ResourceIdRetriever;
use crate::types::Content;
use crate::types::EndPoint;
use crate::types::ResourceId;
use crate::EmeraldError;
use crate::Result;

use EmeraldError::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::content_loader::ContentLoader;

pub struct FileContentLoader {
    resource_id_retriever: Rc<dyn ResourceIdRetriever>,
}

impl FileContentLoader {
    pub fn new(resource_id_retriever: Rc<dyn ResourceIdRetriever>) -> Self {
        Self {
            resource_id_retriever,
        }
    }
}

impl ContentLoader for FileContentLoader {
    fn load(&self, resource_id: &ResourceId) -> Result<Content> {
        let endpoint = self.resource_id_retriever.retrieve(resource_id)?;

        let EndPoint::FileMarkdown(md_path) = endpoint else {
            return Err(NotAMarkdownFile);
        };
        Ok(fs::read_to_string(md_path)?.into())
    }
}

#[cfg(test)]
mod tests {}
