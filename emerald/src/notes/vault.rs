use super::note::Note;

pub trait Vault {
    fn flat_iter(&self) -> std::vec::IntoIter<Note>;
}
