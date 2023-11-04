use std::rc::Rc;

use super::provider::Provider;
use crate::notes;

pub trait ProviderFactory {
    fn create_title_provider(&self) -> Box<dyn Provider<String>>;
    fn create_markdown_provider(&self) -> Box<dyn Provider<String>>;
    fn create_size_provider(&self) -> Box<dyn Provider<u64>>;
    fn create_created_time_provider(&self) -> Box<dyn Provider<i64>>;
    fn create_modified_time_provider(&self) -> Box<dyn Provider<i64>>;
    fn create_linked_note_provider(
        &self,
        note_factory: Rc<dyn notes::NoteFactory>,
    ) -> Box<dyn Provider<Box<dyn Iterator<Item = notes::Note>>>>;
}
