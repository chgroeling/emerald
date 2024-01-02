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

#[cfg(test)]
mod note_updater_tests {
    use super::*;
    use mockall::{predicate::*, *};

    mock! {
        MdContentRetrieverImpl {}
        impl MdContentRetriever for MdContentRetrieverImpl {
            fn retrieve(&self, rid: &ExResourceId) -> &str;
        }
    }

    fn setup_md_content_retriever_mock(inp_str: String) -> Rc<MockMdContentRetrieverImpl> {
        let mut mock_cnt_retriever = MockMdContentRetrieverImpl::new();
        mock_cnt_retriever
            .expect_retrieve()
            .return_const(inp_str.clone());

        mock_cnt_retriever.into()
    }

    #[test]
    fn test_update_note_identiy_without_yaml_frontmatter() {
        let inp_str: String = "\
Test Text
Text Test"
            .into();
        let mock_cnt_retriever = setup_md_content_retriever_mock(inp_str.clone());
        let sut = NoteUpdater::new(mock_cnt_retriever);
        let rid = ExResourceId("ex_resource_id_1".to_string().into_boxed_str());

        let out = sut.update_note(&rid);

        assert_eq!(out, inp_str)
    }

    #[test]
    fn test_update_note_identiy_with_yaml_frontmatter() {
        let inp_str: String = "\
---
yaml1: text1
yaml2: text2
---
Test Text
Text Test"
            .into();
        let mock_cnt_retriever = setup_md_content_retriever_mock(inp_str.clone());
        let sut = NoteUpdater::new(mock_cnt_retriever);
        let rid = ExResourceId("ex_resource_id_1".to_string().into_boxed_str());

        let out = sut.update_note(&rid);
        assert_eq!(out, inp_str)
    }
}
