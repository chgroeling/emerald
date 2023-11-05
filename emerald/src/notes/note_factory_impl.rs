use std::rc::Rc;

use super::note_factory::NoteFactory;
use super::providers::ProviderFactory;
use super::Note;
use crate::model::note;
use crate::types;

#[derive(Clone)]
pub struct NoteFactoryImpl<I: ProviderFactory> {
    provider_factory: I,
    meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>,
}

impl<I: ProviderFactory> NoteFactoryImpl<I> {
    pub fn new(
        provider_factory: I,
        meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>,
    ) -> Self {
        Self {
            provider_factory,
            meta_data_retriever,
        }
    }
}

impl<I: ProviderFactory + Clone + 'static> NoteFactory for NoteFactoryImpl<I> {
    fn create_note(&self, rid: types::ResourceId) -> Note {
        let meta_data = self.meta_data_retriever.retrieve(&rid);
        Note::new(
            rid,
            self.provider_factory.create_title_provider(),
            self.provider_factory.create_markdown_provider(),
            meta_data.size,
            meta_data.created,
            meta_data.modified,
        )
    }
}
