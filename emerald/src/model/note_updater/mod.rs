mod ex_resource_id;
mod md_content_retriever;
mod note_update_command;
pub use self::ex_resource_id::ExResourceId;
pub use self::md_content_retriever::MdContentRetriever;
use crate::markdown::{DefaultMarkdownFrontmatterSplitter, MarkdownFrontmatterSplitter};
use note_update_command::NoteUpdateCommand::*;
use serde_yaml::Value;
use std::rc::Rc;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

trait Command {
    fn execute(&self, note_updater: &mut dyn UpdateYamlEntry);
}
#[derive(Debug, Clone, PartialEq, Hash, Default)]
struct UpdateEntryCommand {
    entry: String,
    value: String,
}

impl Command for UpdateEntryCommand {
    fn execute(&self, note_updater: &mut dyn UpdateYamlEntry) {
        note_updater.update_entry(&self.entry, &self.value);
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Default)]
struct DefaultDoNothingCommand {}

impl Command for DefaultDoNothingCommand {
    fn execute(&self, _note_updater: &mut dyn UpdateYamlEntry) {}
}
trait UpdateYamlEntry {
    fn update_entry(&mut self, entry: &str, value: &str);
}

struct DefaultUpdateYamlEntry {
    val: Value,
}

impl DefaultUpdateYamlEntry {
    fn new(val: Value) -> Self {
        DefaultUpdateYamlEntry { val }
    }

    fn into_value(self) -> Value {
        self.val
    }
}
impl UpdateYamlEntry for DefaultUpdateYamlEntry {
    fn update_entry(&mut self, entry: &str, value: &str) {
        // update yaml
        // ...
        let mut prop = self.val.get_mut(entry).unwrap();
        if let Value::String(string) = &mut prop {
            string.clear();
            string.push_str(value)
        }
    }
}
pub struct NoteUpdater {
    content_retriever: Rc<dyn MdContentRetriever>,
}

impl NoteUpdater {
    pub fn new(content_retriever: Rc<dyn MdContentRetriever>) -> Self {
        Self { content_retriever }
    }

    pub fn update_note(
        &self,
        rid: &ExResourceId,
        cmd: note_update_command::NoteUpdateCommand,
    ) -> String {
        // read content
        let content = self.content_retriever.retrieve(rid);
        let markdown_splitter = DefaultMarkdownFrontmatterSplitter::new();

        // split
        let (yaml, markdown) = markdown_splitter.split(content);

        let yaml_string = match yaml {
            Some(yaml_str) => {
                let res = serde_yaml::from_str::<Value>(yaml_str);
                let val = res.unwrap();
                let mut yaml_updater = DefaultUpdateYamlEntry::new(val);
                let concrete_cmd: Box<dyn Command> = match cmd {
                    UpdateEntry { entry, value } => Box::new(UpdateEntryCommand { entry, value }),
                    DoNothing => Box::new(DefaultDoNothingCommand {}),
                };
                concrete_cmd.execute(&mut yaml_updater);
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
        let sut = NoteUpdater::new(mock_cnt_retriever);
        let rid = ExResourceId("ex_resource_id_1".to_string().into_boxed_str());

        let out = sut.update_note(&rid, DoNothing);

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
        let sut = NoteUpdater::new(mock_cnt_retriever);
        let rid = ExResourceId("ex_resource_id_1".to_string().into_boxed_str());
        let out = sut.update_note(&rid, DoNothing);
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
        let sut = NoteUpdater::new(mock_cnt_retriever);
        let rid = ExResourceId("ex_resource_id_1".to_string().into_boxed_str());

        let out = sut.update_note(
            &rid,
            UpdateEntry {
                entry: "yaml1".into(),
                value: "replace".into(),
            },
        );

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
        let sut = NoteUpdater::new(mock_cnt_retriever);
        let rid = ExResourceId("ex_resource_id_1".to_string().into_boxed_str());
        let out = sut.update_note(
            &rid,
            UpdateEntry {
                entry: "yaml2".into(),
                value: "replace".into(),
            },
        );

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

    #[test]
    fn test_update_note_update_property_with_yaml_frontmatter_insert() {
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
        let out = sut.update_note(
            &rid,
            UpdateEntry {
                entry: "yaml3".into(),
                value: "insert".into(),
            },
        );

        let out_str: String = "\
---
yaml1: text1
yaml2: text2
yaml3: insert
---
Test Text
Text Test"
            .into();
        assert_eq!(out, out_str)
    }
}
