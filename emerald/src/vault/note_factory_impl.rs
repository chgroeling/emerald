use super::note_factory::NoteFactory;
use super::timestamp::Timestamp;
use super::Note;
use crate::markdown;
use crate::markdown::MarkdownAnalyzer;
use crate::model::{content, note};
use crate::types::{self, Content};
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

// TODO: Separate this in one class
fn split(content: &Content) -> (String, String) {
    let md_analyzer = markdown::MarkdownAnalyzerImpl::new();
    let mut md_iter = md_analyzer.analyze(&content.0);
    let mut yaml_str = "".to_string();
    let first_element = md_iter.next();
    let mut start_of_markdown = 0;
    if let Some(md) = first_element {
        if let types::MdBlock::YamlFrontmatter(yaml) = md {
            // markdown starts when yaml ends
            start_of_markdown = yaml.len();
            yaml_str = yaml.to_string();
        }
    }

    return (yaml_str, content.0[start_of_markdown..].to_string());
}

impl NoteFactory for NoteFactoryImpl {
    fn create_note(&self, rid: types::ResourceId) -> Note {
        let meta_data = self.meta_data_retriever.retrieve(&rid);
        let content = self.content_retriever.retrieve(&rid);

        let (yaml_str, markdown) = split(&content);

        Note::new(
            rid,
            meta_data.title.clone(),
            yaml_str,
            meta_data.location.clone(),
            markdown,
            meta_data.size,
            Timestamp(meta_data.created),
            Timestamp(meta_data.modified),
        )
    }
}
