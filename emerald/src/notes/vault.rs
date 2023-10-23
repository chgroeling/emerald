use super::note::Note;
use super::providers::ProviderFactory;
use crate::types;

#[derive(Clone)]
pub struct Vault<U>
where
    U: ProviderFactory,
{
    md_rids: Vec<types::ResourceId>,
    provider_factory: U,
}

impl<U> Vault<U>
where
    U: ProviderFactory,
{
    pub fn new<'a>(
        it_src: impl IntoIterator<Item = &'a types::ResourceId>,
        provider_factory: U,
    ) -> Self {
        let md_rids = it_src.into_iter().cloned().collect();
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
            .iter()
            .map(move |f| Note::new(f.clone(), create_title_p(), create_content_p()))
            .collect();

        note_vec.into_iter()
    }
}
