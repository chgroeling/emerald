use std::rc::Rc;

use crate::{resources::content_queryable::ContentQueryable, types::ResourceId};

use super::content_provider::ContentProvider;

pub struct StdContentProvider<I>
where
    I: ContentQueryable,
{
    content_queryable: Rc<I>,
}

impl<I> StdContentProvider<I>
where
    I: ContentQueryable,
{
    pub fn new(content_queryable: Rc<I>) -> Self {
        Self { content_queryable }
    }
}
impl<I> ContentProvider for StdContentProvider<I>
where
    I: ContentQueryable,
{
    fn get_content(&self, resource_id: &ResourceId) -> String {
        let res = self.content_queryable.query(resource_id.clone()).unwrap();
        (*res.0).clone()
    }
}
