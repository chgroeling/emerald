use std::path::Path;
use std::rc::Rc;

use crate::maps::endpoint_retriever::EndPointRetriever;
use crate::types::meta_data::MetaData;
use crate::types::EndPoint;
use crate::types::ResourceId;
use crate::EmeraldError;
use crate::Result;

use EmeraldError::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::meta_data_loader::MetaDataLoader;

pub struct FileMetaDataLoader {
    ep_retriever: Rc<dyn EndPointRetriever>,
}

impl FileMetaDataLoader {
    pub fn new(ep_retriever: Rc<dyn EndPointRetriever>) -> Self {
        Self { ep_retriever }
    }

    fn get_file_meta_data(&self, path: &Path) -> Result<MetaData> {
        let os_filename = path.file_stem().ok_or(NotAFile)?;
        let file_stem = os_filename.to_str().ok_or(ValueError)?.into();
        Ok(MetaData { file_stem })
    }
}

impl MetaDataLoader for FileMetaDataLoader {
    fn load(&self, resource_id: &ResourceId) -> Result<MetaData> {
        let endpoint = self.ep_retriever.retrieve(resource_id)?;

        #[allow(unreachable_patterns)]
        match endpoint {
            EndPoint::FileMarkdown(path) => self.get_file_meta_data(&path),
            EndPoint::File(path) => self.get_file_meta_data(&path),
            _ => Err(NoMetaData),
        }
    }
}

#[cfg(test)]
mod tests {}
