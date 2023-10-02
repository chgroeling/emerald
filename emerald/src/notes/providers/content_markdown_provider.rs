use std::rc::Rc;

use crate::{resources::content_loader::ContentLoader, types::ResourceId};

use super::markdown_provider::MarkdownProvider;

pub struct ContentMarkdownProvider<I>
where
    I: ContentLoader,
{
    content_queryable: Rc<I>,
}

impl<I> ContentMarkdownProvider<I>
where
    I: ContentLoader,
{
    pub fn new(content_queryable: Rc<I>) -> Self {
        Self { content_queryable }
    }
}
impl<I> MarkdownProvider for ContentMarkdownProvider<I>
where
    I: ContentLoader,
{
    fn get_markdown(&self, resource_id: &ResourceId) -> String {
        let res = self.content_queryable.load(resource_id).unwrap();
        (*res.0).clone()
    }
}
