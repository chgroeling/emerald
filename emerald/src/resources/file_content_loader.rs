use std::fs;
use std::rc::Rc;

use crate::maps::resource_id_queryable::ResourceIdQueryable;
use crate::types::Content;
use crate::types::EndPoint;
use crate::types::ResourceId;
use crate::EmeraldError;
use crate::Result;

use EmeraldError::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::content_queryable::ContentQueryable;

pub struct FileContentLoader {
    resource_id_queryable: Rc<dyn ResourceIdQueryable>,
}

impl FileContentLoader {
    pub fn new(resource_id_queryable: Rc<dyn ResourceIdQueryable>) -> Self {
        Self {
            resource_id_queryable,
        }
    }
}

impl ContentQueryable for FileContentLoader {
    fn query(&self, resource_id: ResourceId) -> Result<Content> {
        let endpoint = self.resource_id_queryable.get(&resource_id)?;

        let EndPoint::FileMarkdown(md_path) = endpoint else {
            return Err(NotAMarkdownFile);
        };
        Ok(fs::read_to_string(md_path)?.into())
    }
}

#[cfg(test)]
mod tests {}
