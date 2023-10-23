use super::endpoint_retriever::EndpointRetriever;
use super::meta_data_loader::MetaDataLoader;
use super::resource_object::ResourceObject;
use crate::error::{EmeraldError::*, Result};
use crate::types;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::path::Path;

#[derive(Clone)]
pub struct FileMetaDataLoader<I>
where
    I: EndpointRetriever,
{
    ep_retriever: I,
}

impl<I> FileMetaDataLoader<I>
where
    I: EndpointRetriever,
{
    pub fn new(ep_retriever: I) -> Self {
        Self { ep_retriever }
    }

    fn get_file_type(&self, path: &Path) -> Result<types::FileType> {
        let os_ext = path.extension().ok_or(NotAFile)?;
        let ext = os_ext.to_str().ok_or(ValueError)?;
        match ext {
            "md" => Ok(types::FileType::Markdown(ext.to_string())),
            "markdown" => Ok(types::FileType::Markdown(ext.to_string())),
            _ => Ok(types::FileType::Unknown(ext.to_string())),
        }
    }

    fn get_file_stem(&self, path: &Path) -> Result<String> {
        let os_filename = path.file_stem().ok_or(NotAFile)?;
        let file_stem = os_filename.to_str().ok_or(ValueError)?.to_string();
        Ok(file_stem)
    }

    fn get_file_meta_data(&self, path: &Path) -> Result<types::MetaData> {
        let file_stem = self.get_file_stem(path)?;
        let file_type = self.get_file_type(path)?;
        Ok(types::MetaData {
            file_stem,
            file_type,
        })
    }
}

impl<I> MetaDataLoader for FileMetaDataLoader<I>
where
    I: EndpointRetriever,
{
    fn load(&self, rid: &types::ResourceId) -> Result<types::MetaData> {
        let ep = self.ep_retriever.retrieve(rid)?;

        #[allow(unreachable_patterns)]
        match ep {
            ResourceObject::File(path) => self.get_file_meta_data(&path),
            _ => Err(NoMetaData),
        }
    }
}

#[cfg(test)]
mod tests {}
