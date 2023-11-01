use super::meta_data_loader::MetaDataLoader;
use super::resource_object::ResourceObject;
use super::resource_object_retriever::ResourceObjectRetriever;
use crate::error::{EmeraldError::*, Result};
use crate::{types, EmeraldError};
use chrono::prelude::*;
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

trait FsMetaData {}
pub struct FsMetaDataImpl();

impl FsMetaDataImpl {
    pub fn get_meta_data_from_fs(&self, path: &Path) -> Result<fs::Metadata> {
        if let Ok(meta_data) = fs::metadata(path) {
            Ok(meta_data)
        } else {
            Err(EmeraldError::NoMetaData)
        }
    }
}
#[derive(Clone)]
pub struct FileMetaDataLoader<I>
where
    I: ResourceObjectRetriever,
{
    ro_retriever: I,
}

impl<I> FileMetaDataLoader<I>
where
    I: ResourceObjectRetriever,
{
    pub fn new(ro_retriever: I) -> Self {
        Self { ro_retriever }
    }

    fn get_file_type(&self, path: &Path) -> Result<types::FileType> {
        let Some(os_ext) = path.extension() else {
            return Ok(types::FileType::NoFileType());
        };

        let ext = os_ext.to_str().ok_or(ValueError)?;
        match ext {
            "md" => Ok(types::FileType::Markdown(ext.to_string())),
            "markdown" => Ok(types::FileType::Markdown(ext.to_string())),
            _ => Ok(types::FileType::Unknown(ext.to_string())),
        }
    }

    fn get_file_stem(&self, path: &Path) -> Result<String> {
        let os_filename = path.file_stem().ok_or(NotAFile(path.into()))?;
        let file_stem = os_filename.to_str().ok_or(ValueError)?.to_string();
        Ok(file_stem)
    }

    fn get_times(&self, path: &Path) -> Result<String> {
        let fs_meta_data = FsMetaDataImpl();
        let metadata = fs_meta_data.get_meta_data_from_fs(path)?;

        let modified = metadata.modified()?;
        let dur = modified.duration_since(UNIX_EPOCH).unwrap();
        let dur_u64 = dur.as_secs();

        let dt: DateTime<Local> = modified.into();

        let dt2 = Local.timestamp_opt(dur_u64 as i64, 0);
        if let Ok(time) = metadata.modified() {
            println!("{dt:?} // {dt2:?}");
        } else {
            println!("Not supported on this platform");
        }
        Ok("cdc".into())
    }

    fn get_file_meta_data(&self, path: &Path) -> Result<types::MetaData> {
        let file_stem = self.get_file_stem(path)?;
        let file_type = self.get_file_type(path)?;
        let file_times = self.get_times(path)?;
        Ok(types::MetaData {
            file_stem,
            file_type,
            modified: 0 as u64,
            created: 0 as u64,
        })
    }
}

impl<I> MetaDataLoader for FileMetaDataLoader<I>
where
    I: ResourceObjectRetriever,
{
    fn load(&self, rid: &types::ResourceId) -> Result<types::MetaData> {
        let ro = self.ro_retriever.retrieve(rid)?;

        #[allow(unreachable_patterns)]
        match ro {
            ResourceObject::File(path) => self.get_file_meta_data(&path),
            _ => Err(NoMetaData),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FileMetaDataLoader;
    use crate::resources::resource_object::ResourceObject;
    use crate::resources::{MetaDataLoader, MockResourceObjectRetriever};
    use crate::types;
    use std::path::PathBuf;

    fn create_test_case(path: PathBuf) -> FileMetaDataLoader<MockResourceObjectRetriever> {
        let mut mock = MockResourceObjectRetriever::new();
        mock.expect_retrieve()
            .returning(move |_f| Ok(ResourceObject::File(path.clone())));
        FileMetaDataLoader::new(mock)
    }

    #[test]
    fn test_load_file_type_is_markdown() {
        let dut = create_test_case("test.md".into());
        let res = dut.load(&types::ResourceId::from("resid0")).unwrap();
        assert_eq!(res.file_type, types::FileType::Markdown("md".into()))
    }

    #[test]
    fn test_load_file_type_is_no_file_type() {
        let dut = create_test_case("test".into());
        let res = dut.load(&types::ResourceId::from("resid0")).unwrap();
        assert_eq!(res.file_type, types::FileType::NoFileType())
    }
}
