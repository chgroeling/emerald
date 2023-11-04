use crate::notes::{self, providers::linked_note_provider::LinkedNoteProvider};
use std::rc::Rc;

use super::{
    content_md_provider::ContentMdProvider, meta_data_provider::MetaDataProvider, provider,
    provider_factory::ProviderFactory,
};
use crate::{model::content, model::note};

#[derive(Clone)]
pub struct ProviderFactoryImpl {
    meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>,
    content_retriever: Rc<dyn content::MdContentRetriever>,
}

impl ProviderFactoryImpl {
    pub fn new(
        meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>,
        content_retriever: Rc<dyn content::MdContentRetriever>,
    ) -> Self {
        Self {
            meta_data_retriever,
            content_retriever,
        }
    }
}

impl ProviderFactory for ProviderFactoryImpl {
    fn create_title_provider(&self) -> Box<dyn provider::Provider<String>> {
        Box::new(MetaDataProvider::new(
            self.meta_data_retriever.clone(),
            |meta_data| meta_data.name.to_owned(),
        ))
    }

    fn create_markdown_provider(&self) -> Box<dyn provider::Provider<String>> {
        Box::new(ContentMdProvider::new(
            self.content_retriever.clone(),
            self.meta_data_retriever.clone(),
        ))
    }

    fn create_size_provider(&self) -> Box<dyn provider::Provider<u64>> {
        Box::new(MetaDataProvider::new(
            self.meta_data_retriever.clone(),
            |meta_data| meta_data.size,
        ))
    }
    fn create_created_time_provider(&self) -> Box<dyn provider::Provider<i64>> {
        Box::new(MetaDataProvider::new(
            self.meta_data_retriever.clone(),
            |meta_data| meta_data.created,
        ))
    }

    fn create_modified_time_provider(&self) -> Box<dyn provider::Provider<i64>> {
        Box::new(MetaDataProvider::new(
            self.meta_data_retriever.clone(),
            |meta_data| meta_data.modified,
        ))
    }

    fn create_linked_note_provider(
        &self,
        note_factory: Box<dyn notes::NoteFactory>,
    ) -> Box<dyn provider::Provider<notes::Note>> {
        Box::new(LinkedNoteProvider::new(
            note_factory,
            self.meta_data_retriever.clone(),
        ))
    }
}
