use std::io;
use std::path::{PathBuf, StripPrefixError};
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum EmeraldError {
    #[error("A vault at the specified position could not be found")]
    VaultNotFound,

    #[error("Strip prefix error")]
    StripPrefixError(#[from] StripPrefixError),

    #[error("General io Error")]
    IoError(#[from] io::Error),

    #[error("No endpoint was found.")]
    EndPointNotFound,

    #[error("The link {0} was not found.")]
    LinkNotFound(String),

    #[error("The path {0} is not a sub-path of {1} ")]
    NoCommonPath(PathBuf, PathBuf),

    #[error("Wrong value was assigned.")]
    ValueError,

    #[error("Failed interpreting a wiki link.")]
    NotAWikiLink,

    #[error("Failed interpreting a resource id.")]
    NotAResourceId,

    #[error("Tried to load content which is not a markdown file")]
    NotAMarkdownFile,

    #[error("unknown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, EmeraldError>;
