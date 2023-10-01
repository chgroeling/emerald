use std::rc::Rc;

use crate::{resources::content_queryable::ContentQueryable, types::ResourceId};

use super::content_provider::ContentProvider;

pub struct StdContentProvider {
    content_queryable: Rc<dyn ContentQueryable>,
}

impl StdContentProvider {
    pub fn new(content_queryable: Rc<dyn ContentQueryable>) -> Self {
        Self { content_queryable }
    }
}
impl ContentProvider for StdContentProvider {
    fn get_content(&self, resource_id: &ResourceId) -> String {
        let res = self.content_queryable.get(resource_id.clone()).unwrap();
        (*res.0).clone()
    }
}
