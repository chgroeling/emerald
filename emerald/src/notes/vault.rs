use std::rc::Rc;

use crate::{
    indexes::ResourceIdsIterable,
    providers::meta_data_title_provider::MetaDataTitleProvider,
    resources::meta_data_loader::MetaDataLoader,
    types::{note::Note, ResourceId},
};

pub struct Vault<I: ResourceIdsIterable>
where
    I::Iter: Iterator<Item = ResourceId>,
{
    md_resource_ids_iter: Rc<I>,
    meta_data_loader: Rc<dyn MetaDataLoader>,
}

impl<I: ResourceIdsIterable> Vault<I>
where
    I::Iter: Iterator<Item = ResourceId>,
{
    pub fn new(md_resource_ids_iter: Rc<I>, meta_data_loader: Rc<dyn MetaDataLoader>) -> Self {
        Self {
            md_resource_ids_iter,
            meta_data_loader,
        }
    }

    pub fn flat_iter(&self) -> std::vec::IntoIter<Note> {
        let create_title_p = || Box::new(MetaDataTitleProvider::new(self.meta_data_loader.clone()));
        let note_vec: Vec<Note> = self
            .md_resource_ids_iter
            .iter()
            .map(move |f| Note::new(f, create_title_p()))
            .collect();

        note_vec.into_iter()
    }
}
