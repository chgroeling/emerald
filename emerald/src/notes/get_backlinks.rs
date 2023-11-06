use super::Note;
use crate::types;

pub trait GetBacklinks {
    fn get_backlinks_of(&self, note: &Note) -> Box<dyn Iterator<Item = types::ResourceId>>;
}
