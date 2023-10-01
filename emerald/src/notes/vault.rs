use std::rc::Rc;

use crate::{
    indexes::ResourceIdsIterable,
    types::{note::Note, ResourceId},
};

use crate::notes::providers::provider_factory::ProviderFactory;

pub struct Vault<I: ResourceIdsIterable>
where
    I::Iter: Iterator<Item = ResourceId>,
{
    md_resource_ids_iter: Rc<I>,
    provider_factory: Rc<dyn ProviderFactory>,
}

impl<I: ResourceIdsIterable> Vault<I>
where
    I::Iter: Iterator<Item = ResourceId>,
{
    pub fn new(md_resource_ids_iter: Rc<I>, provider_factory: Rc<dyn ProviderFactory>) -> Self {
        Self {
            md_resource_ids_iter,
            provider_factory,
        }
    }

    pub fn flat_iter(&self) -> std::vec::IntoIter<Note> {
        let create_title_p = || self.provider_factory.create_title_provider();
        let create_content_p = || self.provider_factory.create_content_provider();
        let note_vec: Vec<Note> = self
            .md_resource_ids_iter
            .iter()
            .map(move |f| Note::new(f, create_title_p(), create_content_p()))
            .collect();

        note_vec.into_iter()
    }
}