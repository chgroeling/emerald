use super::note_factory::NoteFactory;
use super::timestamp::Timestamp;
use super::Note;
use crate::markdown;
use crate::markdown::MarkdownAnalyzer;
use crate::model::{content, note};
use crate::types;
use std::rc::Rc;

#[derive(Clone)]
pub struct NoteFactoryImpl {
    meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>,
    content_retriever: Rc<dyn content::MdContentRetriever>,
}

impl NoteFactoryImpl {
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

impl NoteFactory for NoteFactoryImpl {
    fn create_note(&self, rid: types::ResourceId) -> Note {
        let meta_data = self.meta_data_retriever.retrieve(&rid);
        let content = self.content_retriever.retrieve(&rid);

        // TODO: Move this outside of this class and provide a separate implementation
        let md_analyzer = markdown::MarkdownAnalyzerImpl::new();
        let mut md_iter = md_analyzer.analyze(&content.0);

        let mut yaml_str = "".to_string();
        let first_element = md_iter.next();
        if let Some(md) = first_element {
            if let types::MdBlock::YamlFrontmatter(yaml) = md {
                yaml_str = yaml.to_string();
            }
        }

        Note::new(
            rid,
            meta_data.title.clone(),
            yaml_str,
            meta_data.location.clone(),
            content.0.clone(),
            meta_data.size,
            Timestamp(meta_data.created),
            Timestamp(meta_data.modified),
        )
    }
}
