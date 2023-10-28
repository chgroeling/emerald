use std::rc::Rc;

use super::note::Note;
use super::providers::ProviderFactory;
use crate::model;
use crate::types;

#[derive(Clone)]
pub struct Vault<U, I>
where
    U: ProviderFactory,
    I: Iterator<Item = types::ResourceId>,
{
    md_rids: Rc<dyn model::NotesIterSrc<Iter = I>>,
    provider_factory: U,
}

impl<U, I> Vault<U, I>
where
    U: ProviderFactory,
    I: Iterator<Item = types::ResourceId>,
{
    pub fn new<'a>(md_rids: Rc<dyn model::NotesIterSrc<Iter = I>>, provider_factory: U) -> Self
    where
        I: Iterator<Item = types::ResourceId>,
    {
        Self {
            md_rids,
            provider_factory,
        }
    }

    pub fn flat_iter(&self) -> std::vec::IntoIter<Note> {
        let create_title_p = || self.provider_factory.create_title_provider();
        let create_content_p = || self.provider_factory.create_markdown_provider();
        let note_vec: Vec<Note> = self
            .md_rids
            .create_iter()
            .map(move |f| Note::new(f.clone(), create_title_p(), create_content_p()))
            .collect();

        note_vec.into_iter()
    }
}
