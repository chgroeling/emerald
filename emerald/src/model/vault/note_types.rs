use super::Note;

pub enum NoteTypes<T>
where
    T: std::fmt::Debug + std::hash::Hash + Eq + Clone,
{
    Note(Note),
    ResourceRef(T),
}
