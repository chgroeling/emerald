use super::note_factory::NoteFactory;
use super::providers::ProviderFactory;
use super::Note;
use crate::types;

#[derive(Clone)]
pub struct NoteFactoryImpl<I: ProviderFactory> {
    provider_factory: I,
}

impl<I: ProviderFactory> NoteFactoryImpl<I> {
    pub fn new(provider_factory: I) -> Self {
        Self { provider_factory }
    }
}
impl<I: ProviderFactory> NoteFactory for NoteFactoryImpl<I> {
    fn create_note(&self, rid: types::ResourceId) -> Note {
        Note::new(
            rid,
            self.provider_factory.create_title_provider(),
            self.provider_factory.create_markdown_provider(),
            self.provider_factory.create_size_provider(),
            self.provider_factory.create_created_time_provider(),
            self.provider_factory.create_modified_time_provider(),
        )
    }
}
