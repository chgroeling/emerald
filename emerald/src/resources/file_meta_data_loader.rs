use super::meta_data_loader::MetaDataLoader;
use super::resource_object::ResourceObject;
use super::resource_object_retriever::ResourceObjectRetriever;
use crate::error::{EmeraldError::*, Result};
use crate::types::MetaDataBuilder;
use crate::{types, EmeraldError};
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

#[cfg(test)]
use mockall::{predicate::*, *};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub struct FsMetadata {
    modified: u64,
    created: u64,
}
#[cfg_attr(test, automock)]
pub trait FsMetadataAccess {
    fn get_meta_data_from_fs(&self, path: &Path) -> Result<FsMetadata>;
}
pub struct FsMetadataAccessImpl();
impl FsMetadataAccess for FsMetadataAccessImpl {
    fn get_meta_data_from_fs(&self, path: &Path) -> Result<FsMetadata> {
        if let Ok(meta_data) = fs::metadata(path) {
            let modified = meta_data.modified()?;
            let modified_dur = modified.duration_since(UNIX_EPOCH).unwrap();
            let modified_u64 = modified_dur.as_secs();

            let created = meta_data.created()?;
            let created_dur = created.duration_since(UNIX_EPOCH).unwrap();
            let created_u64 = created_dur.as_secs();

            Ok(FsMetadata {
                modified: modified_u64,
                created: created_u64,
            })
        } else {
            Err(EmeraldError::NoMetaData)
        }
    }
}
#[derive(Clone)]
pub struct FileMetaDataLoaderImpl<I, U = FsMetadataAccessImpl>
where
    I: ResourceObjectRetriever,
    U: FsMetadataAccess,
{
    ro_retriever: I,
    fs_meta_data_access: U,
}

impl<I, U> FileMetaDataLoaderImpl<I, U>
where
    I: ResourceObjectRetriever,
    U: FsMetadataAccess,
{
    pub fn new(ro_retriever: I, fs_meta_data_access: U) -> Self {
        Self {
            ro_retriever,
            fs_meta_data_access,
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

    fn get_file_timestamps(&self, path: &Path) -> Result<(i64, i64)> {
        let metadata = self.fs_meta_data_access.get_meta_data_from_fs(path)?;

        Ok((metadata.modified as i64, metadata.created as i64))
    }

    fn get_file_meta_data(&self, path: &Path) -> Result<types::MetaData> {
        let file_stem = self.get_file_stem(path)?;
        let file_type = self.get_file_type(path)?;
        let (modified, created) = self.get_file_timestamps(path)?;

        let builder = MetaDataBuilder::new()
            .set_file_stem(file_stem)
            .set_file_type(file_type)
            .set_created(created)
            .set_modified(modified);
        Ok(builder.build())
    }
}

impl<I, U> MetaDataLoader for FileMetaDataLoaderImpl<I, U>
where
    I: ResourceObjectRetriever,
    U: FsMetadataAccess,
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

pub struct FileMetaDataLoader<I>
where
    I: ResourceObjectRetriever,
{
    imp: FileMetaDataLoaderImpl<I>,
}

impl<I> FileMetaDataLoader<I>
where
    I: ResourceObjectRetriever,
{
    pub fn new(ro_retriever: I) -> Self {
        Self {
            imp: FileMetaDataLoaderImpl::new(ro_retriever, FsMetadataAccessImpl()),
        }
    }
}

impl<I> MetaDataLoader for FileMetaDataLoader<I>
where
    I: ResourceObjectRetriever,
{
    fn load(&self, rid: &types::ResourceId) -> Result<types::MetaData> {
        self.imp.load(rid)
    }
}

#[cfg(test)]
mod tests {
    use super::FileMetaDataLoaderImpl;
    use super::MockFsMetadataAccess;
    use crate::resources::file_meta_data_loader::FsMetadata;
    use crate::resources::resource_object::ResourceObject;
    use crate::resources::{MetaDataLoader, MockResourceObjectRetriever};
    use crate::types;
    use std::path::PathBuf;

    fn create_test_case(
        path: PathBuf,
    ) -> FileMetaDataLoaderImpl<MockResourceObjectRetriever, MockFsMetadataAccess> {
        let mut mock = MockResourceObjectRetriever::new();
        mock.expect_retrieve()
            .returning(move |_f| Ok(ResourceObject::File(path.clone())));

        let mut mock_fs_access = MockFsMetadataAccess::new();
        mock_fs_access
            .expect_get_meta_data_from_fs()
            .returning(|_| {
                Ok(FsMetadata {
                    modified: 0,
                    created: 0,
                })
            });
        FileMetaDataLoaderImpl::new(mock, mock_fs_access)
    }

    #[test]
    fn test_load_file_type_is_markdown() {
        let dut = create_test_case("test.md".into());
        let res = dut.load(&types::ResourceId::from("resid0")).unwrap();
        assert_eq!(res.file_type, types::FileType::Markdown("md".into()));
    }

    #[test]
    fn test_load_file_type_is_no_file_type() {
        let dut = create_test_case("test".into());
        let res = dut.load(&types::ResourceId::from("resid0")).unwrap();
        assert_eq!(res.file_type, types::FileType::NoFileType())
    }
}
