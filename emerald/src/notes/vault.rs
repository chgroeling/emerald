use std::rc::Rc;

use crate::{
    indexes::ResourceIdsIterable,
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
        let out_vec: Vec<Note> = self
            .md_resource_ids_iter
            .iter()
            .map(|f| Note::new(f, self.meta_data_loader.clone()))
            .collect();

        out_vec.into_iter()
    }
}
