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
    size: u64,
    modified: u64,
    created: u64,
}
#[cfg_attr(test, automock)]
pub trait FsMetadataAccess {
    fn get_meta_data_from_fs(&self, path: &Path) -> Result<FsMetadata>;
}
pub struct DefaultFsMetadataAccess();
impl FsMetadataAccess for DefaultFsMetadataAccess {
    fn get_meta_data_from_fs(&self, path: &Path) -> Result<FsMetadata> {
        if let Ok(meta_data) = fs::metadata(path) {
            if !meta_data.is_file() {
                return Err(EmeraldError::NotAFile(path.to_owned()));
            }
            let modified = meta_data.modified()?;
            let modified_dur = modified.duration_since(UNIX_EPOCH).unwrap();
            let modified_u64 = modified_dur.as_secs();

            let created = meta_data.created()?;
            let created_dur = created.duration_since(UNIX_EPOCH).unwrap();
            let created_u64 = created_dur.as_secs();

            Ok(FsMetadata {
                size: meta_data.len(),
                modified: modified_u64,
                created: created_u64,
            })
        } else {
            Err(EmeraldError::NoMetaData)
        }
    }
}
#[derive(Clone)]
pub struct FileMetaDataLoader<I, U = DefaultFsMetadataAccess>
where
    I: ResourceObjectRetriever,
    U: FsMetadataAccess,
{
    ro_retriever: I,
    fs_meta_data_access: U,
}

impl<I, U> FileMetaDataLoader<I, U>
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

    fn get_file_meta_data(&self, path: &Path) -> Result<types::MetaData> {
        // get meta data from filesystem
        let fs_meta_data = self.fs_meta_data_access.get_meta_data_from_fs(path)?;

        // get name of file
        let os_filename = path.file_stem().ok_or(NotAFile(path.into()))?;
        let name = os_filename.to_str().ok_or(ValueError)?.to_string();

        // determine resource type
        let resource_type = if let Some(os_ext) = path.extension() {
            let ext = os_ext.to_str().ok_or(ValueError)?;

            match ext {
                "md" => types::ResourceType::Markdown(ext.to_string()),
                "markdown" => types::ResourceType::Markdown(ext.to_string()),
                _ => types::ResourceType::Unknown(ext.to_string()),
            }
        } else {
            types::ResourceType::NoType()
        };

        let builder = MetaDataBuilder::new()
            .set_name(name)
            .set_size(fs_meta_data.size)
            .set_resource_type(resource_type)
            .set_created(fs_meta_data.created as i64)
            .set_modified(fs_meta_data.modified as i64);
        Ok(builder.build())
    }
}

impl<I, U> MetaDataLoader for FileMetaDataLoader<I, U>
where
    I: ResourceObjectRetriever,
    U: FsMetadataAccess,
{
    fn load(&self, rid: &types::ResourceId) -> Result<types::MetaData> {
        let ro = self.ro_retriever.retrieve(rid)?;
        #[allow(irrefutable_let_patterns)]
        if let ResourceObject::File(path) = ro {
            self.get_file_meta_data(&path)
        } else {
            Err(EmeraldError::NoMetaData)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FileMetaDataLoader;
    use super::MockFsMetadataAccess;
    use crate::resources::file_meta_data_loader::FsMetadata;
    use crate::resources::resource_object::ResourceObject;
    use crate::resources::{MetaDataLoader, MockResourceObjectRetriever};
    use crate::types;
    use std::path::PathBuf;

    fn create_test_case(
        path: PathBuf,
    ) -> FileMetaDataLoader<MockResourceObjectRetriever, MockFsMetadataAccess> {
        let mut mock = MockResourceObjectRetriever::new();
        mock.expect_retrieve()
            .returning(move |_f| Ok(ResourceObject::File(path.clone())));

        let mut mock_fs_access = MockFsMetadataAccess::new();
        mock_fs_access
            .expect_get_meta_data_from_fs()
            .returning(|_| {
                Ok(FsMetadata {
                    size: 0,
                    modified: 0,
                    created: 0,
                })
            });
        FileMetaDataLoader::new(mock, mock_fs_access)
    }

    #[test]
    fn test_load_file_type_is_markdown() {
        let dut = create_test_case("test.md".into());
        let res = dut.load(&types::ResourceId::from("resid0")).unwrap();
        assert_eq!(
            res.resource_type,
            types::ResourceType::Markdown("md".into())
        );
    }

    #[test]
    fn test_load_file_type_is_no_file_type() {
        let dut = create_test_case("test".into());
        let res = dut.load(&types::ResourceId::from("resid0")).unwrap();
        assert_eq!(res.resource_type, types::ResourceType::NoType())
    }
}
