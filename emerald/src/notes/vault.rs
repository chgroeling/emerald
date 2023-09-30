use std::rc::Rc;

use crate::{
    indexes::ResourceIdsIterable,
    types::{note::Note, ResourceId},
};

pub struct Vault<IRes: Iterator<Item = ResourceId>> {
    md_resource_ids_iter: Rc<dyn ResourceIdsIterable<Iter = IRes>>,
}

impl<IRes: Iterator<Item = ResourceId>> Vault<IRes> {
    pub fn new(md_resource_ids_iter: Rc<dyn ResourceIdsIterable<Iter = IRes>>) -> Self {
        Self {
            md_resource_ids_iter,
        }
    }

    pub fn flat_iter(&self) -> std::vec::IntoIter<Note> {
        let out_vec: Vec<Note> = self
            .md_resource_ids_iter
            .iter()
            .map(|f| Note::new(f))
            .collect();

        out_vec.into_iter()
    }
}
