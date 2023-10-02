use std::rc::Rc;

use crate::{resources::content_loader::ContentLoader, types::ResourceId};

use super::md_provider::MdProvider;

pub struct ContentMdProvider<I>
where
    I: ContentLoader,
{
    content_retriever: Rc<I>,
}

impl<I> ContentMdProvider<I>
where
    I: ContentLoader,
{
    pub fn new(content_retriever: Rc<I>) -> Self {
        Self { content_retriever }
    }
}
impl<I> MdProvider for ContentMdProvider<I>
where
    I: ContentLoader,
{
    fn get_markdown(&self, resource_id: &ResourceId) -> String {
        let res = self.content_retriever.load(resource_id).unwrap();
        (*res.0).clone()
    }
}
