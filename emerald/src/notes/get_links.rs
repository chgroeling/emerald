use super::Note;
use crate::types;

pub enum GetLinksResult {
    LinkToNote(types::ResourceId),
    LinkToResource(types::ResourceId),
}

pub trait GetLinks {
    fn get_links_of(&self, note: &Note) -> Box<dyn Iterator<Item = GetLinksResult>>;
}
