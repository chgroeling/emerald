mod ex_resource_id;
mod md_content_retriever;

pub use self::ex_resource_id::ExResourceId;
pub use self::md_content_retriever::MdContentRetriever;
use crate::markdown::MarkdownFrontMatterSplitter;
use crate::types;
use std::rc::Rc;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub struct NoteUpdater {
    content_retriever: Rc<dyn MdContentRetriever>,
}

impl NoteUpdater {
    pub fn new(content_retriever: Rc<dyn MdContentRetriever>) -> Self {
        Self { content_retriever }
    }

    pub fn update_note(&self, rid: &ExResourceId) -> String {
        // read content
        let content = self.content_retriever.retrieve(rid);
        let markdown_splitter = MarkdownFrontMatterSplitter::new();

        // split
        let (yaml_str, markdown) = markdown_splitter.split(content);
        let res = serde_yaml::from_str::<types::DocumentMetadata>(yaml_str);
        let yaml_data = match res {
            Ok(yaml_meta_data) => yaml_meta_data,
            Err(err) => {
                warn!(
                    "Invalid yaml found in {:?}\nError: {}\n{}",
                    rid, err, yaml_str
                );
                types::DocumentMetadata::default()
            }
        };

        // update yaml
        // ...

        // Output
        yaml_str.to_string() + markdown
    }
}
