use super::VaultResourceId;

pub trait NotesIterSrc {
    type Iter: Iterator<Item = VaultResourceId>;
    fn create_iter(&self) -> Self::Iter;
}
