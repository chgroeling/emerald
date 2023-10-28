use std::rc::Rc;

use super::note::Note;
use super::providers::ProviderFactory;
use crate::model::note_model;
use crate::types;

#[derive(Clone)]
pub struct Vault<U, I>
where
    U: ProviderFactory,
    I: Iterator<Item = types::ResourceId>,
{
    notes_iter_src: Rc<dyn note_model::NotesIterSrc<Iter = I>>,
    provider_factory: U,
}

impl<U, I> Vault<U, I>
where
    U: ProviderFactory,
    I: Iterator<Item = types::ResourceId>,
{
    pub fn new(
        notes_iter_src: Rc<dyn note_model::NotesIterSrc<Iter = I>>,
        provider_factory: U,
    ) -> Self
    where
        I: Iterator<Item = types::ResourceId>,
    {
        Self {
            notes_iter_src,
            provider_factory,
        }
    }

    pub fn flat_iter(&self) -> std::vec::IntoIter<Note> {
        let create_title_p = || self.provider_factory.create_title_provider();
        let create_content_p = || self.provider_factory.create_markdown_provider();
        let note_vec: Vec<Note> = self
            .notes_iter_src
            .create_iter()
            .map(move |f| Note::new(f.clone(), create_title_p(), create_content_p()))
            .collect();

        note_vec.into_iter()
    }
}
