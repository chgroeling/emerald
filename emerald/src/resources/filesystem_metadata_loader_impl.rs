use super::filesystem_metadata_loader::FilesystemMetadataLoader;
use super::resource_object::ResourceObject;
use super::resource_object_retriever::ResourceObjectRetriever;
use crate::error::{EmeraldError::*, Result};
use crate::types::FilesystemMetadataBuilder;
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
pub struct FsMetadataAccessImpl();
impl FsMetadataAccess for FsMetadataAccessImpl {
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
            Err(EmeraldError::NoMetadata)
        }
    }
}
#[derive(Clone)]
pub struct FilesystemMetadataLoaderImpl<I, U = FsMetadataAccessImpl>
where
    I: ResourceObjectRetriever,
    U: FsMetadataAccess,
{
    ro_retriever: I,
    fs_meta_data_access: U,
}

impl<I, U> FilesystemMetadataLoaderImpl<I, U>
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

    fn get_file_meta_data(&self, path: &Path) -> Result<types::FilesystemMetadata> {
        // get meta data from filesystem
        let fs_meta_data = self.fs_meta_data_access.get_meta_data_from_fs(path)?;

        // determine resource type
        let resource_type = if let Some(os_ext) = path.extension() {
            let ext = os_ext.to_str().ok_or(ValueError)?;

            match ext {
                "md" => types::ResourceType::Markdown(),
                "markdown" => types::ResourceType::Markdown(),
                _ => types::ResourceType::Unknown(),
            }
        } else {
            types::ResourceType::NoType()
        };

        let builder = FilesystemMetadataBuilder::new()
            .set_path(path.to_owned())
            .set_size(fs_meta_data.size)
            .set_resource_type(resource_type)
            .set_created(fs_meta_data.created as i64)
            .set_modified(fs_meta_data.modified as i64);
        Ok(builder.build())
    }
}

impl<I, U> FilesystemMetadataLoader for FilesystemMetadataLoaderImpl<I, U>
where
    I: ResourceObjectRetriever,
    U: FsMetadataAccess,
{
    fn load(&self, rid: &types::ResourceId) -> Result<types::FilesystemMetadata> {
        let ro = self.ro_retriever.retrieve(rid)?;
        #[allow(irrefutable_let_patterns)]
        if let ResourceObject::File(path) = ro {
            self.get_file_meta_data(&path)
        } else {
            Err(EmeraldError::NoMetadata)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FilesystemMetadataLoaderImpl;
    use super::MockFsMetadataAccess;
    use crate::resources::filesystem_metadata_loader_impl::FsMetadata;
    use crate::resources::resource_object::ResourceObject;
    use crate::resources::{FilesystemMetadataLoader, MockResourceObjectRetriever};
    use crate::types;
    use std::path::PathBuf;

    fn create_test_case(
        path: PathBuf,
    ) -> FilesystemMetadataLoaderImpl<MockResourceObjectRetriever, MockFsMetadataAccess> {
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
        FilesystemMetadataLoaderImpl::new(mock, mock_fs_access)
    }

    #[test]
    fn test_load_file_type_is_markdown() {
        let dut = create_test_case("test.md".into());
        let res = dut.load(&types::ResourceId::from("resid0")).unwrap();
        assert_eq!(res.resource_type, types::ResourceType::Markdown());
    }
    /*
        #[test]
        fn test_load_title_in_nfc() {
            // attention: the öä in the filename have different unicode representations
            let dut = create_test_case("testöä.md".into());
            let res = dut.load(&types::ResourceId::from("resid0")).unwrap();
            assert_eq!(res.name, "testöä");
        }
    */
    #[test]
    fn test_load_file_type_is_no_file_type() {
        let dut = create_test_case("test".into());
        let res = dut.load(&types::ResourceId::from("resid0")).unwrap();
        assert_eq!(res.resource_type, types::ResourceType::NoType())
    }
}
