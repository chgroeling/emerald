use super::meta_data_loader::MetaDataLoader;
use super::resource_object::ResourceObject;
use super::resource_object_retriever::ResourceObjectRetriever;
use crate::error::{EmeraldError::*, Result};
use crate::{types, EmeraldError};
use chrono::prelude::*;
#[cfg(test)]
use mockall::{predicate::*, *};
use std::fs;
use std::marker::PhantomData;
use std::path::Path;
use std::time::UNIX_EPOCH;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[cfg_attr(test, automock)]
pub trait FsMetaData {
    fn get_meta_data_from_fs(path: &Path) -> Result<(u64, u64)>;
}
pub struct FsMetaDataImpl();

impl FsMetaDataImpl {
    pub fn get_meta_data_from_fs(path: &Path) -> Result<(u64, u64)> {
        if let Ok(meta_data) = fs::metadata(path) {
            let modified = meta_data.modified()?;
            let modified_dur = modified.duration_since(UNIX_EPOCH).unwrap();
            let modified_u64 = modified_dur.as_secs();

            let created = meta_data.created()?;
            let created_dur = created.duration_since(UNIX_EPOCH).unwrap();
            let created_u64 = created_dur.as_secs();

            Ok((modified_u64, created_u64))
        } else {
            Err(EmeraldError::NoMetaData)
        }
    }
}

impl FsMetaData for FsMetaDataImpl {
    fn get_meta_data_from_fs(path: &Path) -> Result<(u64, u64)> {
        Self::get_meta_data_from_fs(path)
    }
}
#[derive(Clone)]
pub struct FileMetaDataLoaderImpl<I, U = FsMetaDataImpl>
where
    I: ResourceObjectRetriever,
    U: FsMetaData,
{
    ro_retriever: I,
    phantom_fs_md: PhantomData<U>,
}

impl<I, U> FileMetaDataLoaderImpl<I, U>
where
    I: ResourceObjectRetriever,
    U: FsMetaData,
{
    pub fn new(ro_retriever: I) -> Self {
        Self {
            ro_retriever,
            phantom_fs_md: PhantomData::default(),
        }
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
        let metadata = U::get_meta_data_from_fs(path)?;
        let dt2 = Local.timestamp_opt(metadata.0 as i64, 0);
        println!("// {dt2:?}");

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

impl<I, U> MetaDataLoader for FileMetaDataLoaderImpl<I, U>
where
    I: ResourceObjectRetriever,
    U: FsMetaData,
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

pub type FileMetaDataLoader<I> = FileMetaDataLoaderImpl<I, FsMetaDataImpl>;

#[cfg(test)]
mod tests {
    use super::FileMetaDataLoaderImpl;
    use super::MockFsMetaData;
    use crate::resources::resource_object::ResourceObject;
    use crate::resources::{MetaDataLoader, MockResourceObjectRetriever};
    use crate::types;
    use std::path::PathBuf;

    fn create_test_case(
        path: PathBuf,
    ) -> FileMetaDataLoaderImpl<MockResourceObjectRetriever, MockFsMetaData> {
        let mut mock = MockResourceObjectRetriever::new();
        mock.expect_retrieve()
            .returning(move |_f| Ok(ResourceObject::File(path.clone())));

        FileMetaDataLoaderImpl::new(mock)
    }

    #[test]
    fn test_load_file_type_is_markdown() {
        let dut = create_test_case("test.md".into());
        let ctx = MockFsMetaData::get_meta_data_from_fs_context();
        ctx.expect().returning(|_| Ok((0, 0)));

        let res = dut.load(&types::ResourceId::from("resid0")).unwrap();
        assert_eq!(res.file_type, types::FileType::Markdown("md".into()));
    }

    #[test]
    fn test_load_file_type_is_no_file_type() {
        let dut = create_test_case("test".into());
        let ctx = MockFsMetaData::get_meta_data_from_fs_context();
        ctx.expect().returning(|_| Ok((0, 0)));
        let res = dut.load(&types::ResourceId::from("resid0")).unwrap();
        assert_eq!(res.file_type, types::FileType::NoFileType())
    }
}
