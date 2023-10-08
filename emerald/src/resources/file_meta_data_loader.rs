use std::path::Path;

use crate::resources::endpoint_resolver::EndPointResolver;
use crate::types::meta_data::FileType;
use crate::types::meta_data::MetaData;
use crate::types::EndPoint;
use crate::types::ResourceId;
use crate::EmeraldError;
use crate::Result;

use EmeraldError::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::meta_data_loader::MetaDataLoader;

#[derive(Clone)]
pub struct FileMetaDataLoader<I>
where
    I: EndPointResolver,
{
    ep_retriever: I,
}

impl<I> FileMetaDataLoader<I>
where
    I: EndPointResolver,
{
    pub fn new(ep_retriever: I) -> Self {
        Self { ep_retriever }
    }

    fn get_file_type(&self, path: &Path) -> Result<FileType> {
        let os_ext = path.extension().ok_or(NotAFile)?;
        let ext = os_ext.to_str().ok_or(ValueError)?;
        match ext {
            "md" => Ok(FileType::Markdown(ext.to_string())),
            "markdown" => Ok(FileType::Markdown(ext.to_string())),
            _ => Ok(FileType::Unknown(ext.to_string())),
        }
    }

    fn get_file_stem(&self, path: &Path) -> Result<String> {
        let os_filename = path.file_stem().ok_or(NotAFile)?;
        let file_stem = os_filename.to_str().ok_or(ValueError)?.to_string();
        Ok(file_stem)
    }

    fn get_file_meta_data(&self, path: &Path) -> Result<MetaData> {
        let file_stem = self.get_file_stem(path)?;
        let file_type = self.get_file_type(path)?;
        Ok(MetaData {
            file_stem,
            file_type,
        })
    }
}

impl<I> MetaDataLoader for FileMetaDataLoader<I>
where
    I: EndPointResolver,
{
    fn load(&self, resource_id: &ResourceId) -> Result<MetaData> {
        let ep = self.ep_retriever.resolve(resource_id)?;

        #[allow(unreachable_patterns)]
        match ep {
            EndPoint::FileMarkdown(path) => self.get_file_meta_data(&path),
            EndPoint::FileUnknown(path) => self.get_file_meta_data(&path),
            _ => Err(NoMetaData),
        }
    }
}

#[cfg(test)]
mod tests {}
