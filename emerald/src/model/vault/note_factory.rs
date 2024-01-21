use super::{uid_trait::UidTrait, Note};
pub trait NoteFactory<U>
where
    U: UidTrait,
{
    fn create_note(&self, uid: &U) -> Note<U>;
}
