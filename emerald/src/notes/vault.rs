use std::rc::Rc;

use crate::{
    indexes::ResourceIdsIterable,
    resources::meta_data_loader::{self, MetaDataLoader},
    types::{note::Note, ResourceId},
};

pub struct Vault<IRes: Iterator<Item = ResourceId>> {
    md_resource_ids_iter: Rc<dyn ResourceIdsIterable<Iter = IRes>>,
    meta_data_loader: Rc<dyn MetaDataLoader>,
}

impl<IRes: Iterator<Item = ResourceId>> Vault<IRes> {
    pub fn new(
        md_resource_ids_iter: Rc<dyn ResourceIdsIterable<Iter = IRes>>,
        meta_data_loader: Rc<dyn MetaDataLoader>,
    ) -> Self {
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
