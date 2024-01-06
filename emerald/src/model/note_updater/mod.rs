mod ex_resource_id;
mod md_content_retriever;

pub use self::ex_resource_id::ExResourceId;
pub use self::md_content_retriever::MdContentRetriever;
use crate::markdown::{DefaultMarkdownFrontmatterSplitter, MarkdownFrontmatterSplitter};
use std::rc::Rc;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use serde_yaml::Value;

trait ChangeCommand {}
#[derive(Debug, Clone, PartialEq, Hash, Default)]
pub struct DefaultChangeCommand {
    change: bool,
    entry: String,
    value: String,
}

impl DefaultChangeCommand {
    fn execute(&self, note_updater: &mut dyn YamlUpdater) {
        if self.change {
            note_updater.update_entry(self.entry.clone(), self.value.clone());
        }
    }
}
trait YamlUpdater {
    fn update_entry(&mut self, entry: String, value: String);
}

struct DefaultYamlUpdater {
    val: Value,
}

impl DefaultYamlUpdater {
    fn new(val: Value) -> Self {
        DefaultYamlUpdater { val }
    }

    fn into_value(self) -> Value {
        self.val
    }
}
impl YamlUpdater for DefaultYamlUpdater {
    fn update_entry(&mut self, entry: String, value: String) {
        // update yaml
        // ...
        let mut prop = self.val.get_mut(entry).unwrap();
        if let Value::String(string) = &mut prop {
            string.clear();
            string.push_str(value.as_str())
        }
    }
}
pub struct DefaultNoteUpdater {
    content_retriever: Rc<dyn MdContentRetriever>,
}

impl DefaultNoteUpdater {
    pub fn new(content_retriever: Rc<dyn MdContentRetriever>) -> Self {
        Self { content_retriever }
    }

    pub fn update_note(&self, rid: &ExResourceId, cmd: DefaultChangeCommand) -> String {
        // read content
        let content = self.content_retriever.retrieve(rid);
        let markdown_splitter = DefaultMarkdownFrontmatterSplitter::new();

        // split
        let (yaml, markdown) = markdown_splitter.split(content);

        let yaml_string = match yaml {
            Some(yaml_str) => {
                let res = serde_yaml::from_str::<Value>(yaml_str);
                let jkl = res.unwrap();
                let mut yaml_updater = DefaultYamlUpdater::new(jkl);
                cmd.execute(&mut yaml_updater);
                let own_yaml = yaml_updater.into_value();
                let new_yaml = serde_yaml::to_string(&own_yaml).unwrap();

                "---\n".to_string() + new_yaml.as_str() + "---\n"
            }
            None => "".to_string(),
        };

        // Output
        yaml_string + markdown
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
    fn test_update_note_identity_without_yaml_frontmatter() {
        let inp_str: String = "\
Test Text
Text Test"
            .into();
        let mock_cnt_retriever = setup_md_content_retriever_mock(inp_str.clone());
        let sut = DefaultNoteUpdater::new(mock_cnt_retriever);
        let rid = ExResourceId("ex_resource_id_1".to_string().into_boxed_str());

        let out = sut.update_note(&rid, Default::default());

        assert_eq!(out, inp_str)
    }

    #[test]
    fn test_update_note_identity_with_yaml_frontmatter() {
        let inp_str: String = "\
---
yaml1: text1
yaml2: text2
---
Test Text
Text Test"
            .into();
        let mock_cnt_retriever = setup_md_content_retriever_mock(inp_str.clone());
        let sut = DefaultNoteUpdater::new(mock_cnt_retriever);
        let rid = ExResourceId("ex_resource_id_1".to_string().into_boxed_str());

        let out = sut.update_note(&rid, Default::default());
        assert_eq!(out, inp_str)
    }

    #[test]
    fn test_update_note_update_property_with_yaml_frontmatter() {
        let inp_str: String = "\
---
yaml1: text1
yaml2: text2
---
Test Text
Text Test"
            .into();
        let mock_cnt_retriever = setup_md_content_retriever_mock(inp_str.clone());
        let sut = DefaultNoteUpdater::new(mock_cnt_retriever);
        let rid = ExResourceId("ex_resource_id_1".to_string().into_boxed_str());

        let cmd = DefaultChangeCommand {
            change: true,
            entry: "yaml1".into(),
            value: "replace".into(),
        };
        let out = sut.update_note(&rid, cmd);

        let out_str: String = "\
---
yaml1: replace
yaml2: text2
---
Test Text
Text Test"
            .into();
        assert_eq!(out, out_str)
    }

    #[test]
    fn test_update_note_update_property_with_yaml_frontmatter_2() {
        let inp_str: String = "\
---
yaml1: text1
yaml2: text2
---
Test Text
Text Test"
            .into();
        let mock_cnt_retriever = setup_md_content_retriever_mock(inp_str.clone());
        let sut = DefaultNoteUpdater::new(mock_cnt_retriever);
        let rid = ExResourceId("ex_resource_id_1".to_string().into_boxed_str());

        let cmd = DefaultChangeCommand {
            change: true,
            entry: "yaml2".into(),
            value: "replace".into(),
        };
        let out = sut.update_note(&rid, cmd);

        let out_str: String = "\
---
yaml1: text1
yaml2: replace
---
Test Text
Text Test"
            .into();
        assert_eq!(out, out_str)
    }
}
