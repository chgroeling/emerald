use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmeraldError {
    #[error("A vault at the specified position could not be found")]
    VaultNotFound,

    #[error("General io Error")]
    IoError(#[from] io::Error),

    #[error("No Resource Object was found.")]
    ResourceObjectNotFound,

    #[error("No meta data is available.")]
    NoMetadata,

    #[error("The link {0} was not found.")]
    LinkNotFound(String),

    #[error("The path {0} is not a sub-path of {1} ")]
    NoCommonPath(PathBuf, PathBuf),

    #[error("Wrong value was assigned.")]
    ValueError,

    #[error("Keys are not unique")]
    NotUnique,

    #[error("Failed interpreting a wiki link.")]
    NotAWikiLink,

    #[error("Failed interpreting a resource id.")]
    NotAResourceId,

    #[error("Tried to handle something as a markdown file which was none")]
    NotAMarkdownFile,

    #[error("Tried to handle something as a file which was none")]
    NotAFile(PathBuf),

    #[error("The ResourceObject {0} has no assigned ResourceId")]
    ResourceObjectHasNoResourceId(String),

    #[error("unknown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, EmeraldError>;
