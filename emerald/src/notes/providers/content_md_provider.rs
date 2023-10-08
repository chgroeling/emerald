use crate::{
    resources::{content_loader::ContentLoader, meta_data_loader::MetaDataLoader},
    types::{meta_data::FileType, ResourceId},
};

use super::md_provider::MdProvider;

pub struct ContentMdProvider<T, U>
where
    T: ContentLoader,
    U: MetaDataLoader,
{
    content_loader: T,
    meta_data_loader: U,
}

impl<I, U> ContentMdProvider<I, U>
where
    I: ContentLoader,
    U: MetaDataLoader,
{
    pub fn new(content_loader: I, meta_data_loader: U) -> Self {
        Self {
            content_loader,
            meta_data_loader,
        }
    }
}
impl<I, U> MdProvider for ContentMdProvider<I, U>
where
    I: ContentLoader,
    U: MetaDataLoader,
{
    fn get_markdown(&self, resource_id: &ResourceId) -> String {
        let meta_data = self.meta_data_loader.load(resource_id).unwrap();

        // do not allow anything other than markdown files pass this point
        let FileType::Markdown(_) = meta_data.file_type else {
            panic!("Not a markdown file {:?}", meta_data)
        };

        let res = self.content_loader.load(resource_id).unwrap();
        (*res.0).clone()
    }
}
