use crate::error::Result;
use crate::types;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait FilesystemMetadataLoader {
    fn load(&self, rid: &types::ResourceId) -> Result<types::FilesystemMetadata>;
}
