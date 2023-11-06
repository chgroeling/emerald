use super::Note;

pub trait GetLinks {
    fn get_links_of(&self, note: &Note) -> Box<dyn Iterator<Item = Note>>;
}
