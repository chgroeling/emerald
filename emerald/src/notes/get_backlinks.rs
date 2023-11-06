use super::Note;

pub trait GetBacklinks {
    fn get_backlinks_of(&self, note: &Note) -> Box<dyn Iterator<Item = Note>>;
}
