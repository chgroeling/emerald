use std::rc::Rc;

use super::note::Note;
use super::providers::ProviderFactory;
use crate::model::note;
use crate::{types, Vault};

#[derive(Clone)]
pub struct VaultImpl<U, I>
where
    U: ProviderFactory,
    I: Iterator<Item = types::ResourceId>,
{
    notes_iter_src: Rc<dyn note::NotesIterSrc<Iter = I>>,
    provider_factory: U,
}

impl<U, I> VaultImpl<U, I>
where
    U: ProviderFactory,
    I: Iterator<Item = types::ResourceId>,
{
    pub fn new(notes_iter_src: Rc<dyn note::NotesIterSrc<Iter = I>>, provider_factory: U) -> Self
    where
        I: Iterator<Item = types::ResourceId>,
    {
        Self {
            notes_iter_src,
            provider_factory,
        }
    }

    pub fn flat_iter(&self) -> std::vec::IntoIter<Note> {
        let title_p = || self.provider_factory.create_title_provider();
        let content_p = || self.provider_factory.create_markdown_provider();
        let note_vec: Vec<Note> = self
            .notes_iter_src
            .create_iter()
            .map(move |rid| Note::new(rid, title_p(), content_p()))
            .collect();

        note_vec.into_iter()
    }
}

impl<U, I> Vault for VaultImpl<U, I>
where
    U: ProviderFactory,
    I: Iterator<Item = types::ResourceId>,
{
    fn flat_iter(&self) -> std::vec::IntoIter<Note> {
        self.flat_iter()
    }
}
