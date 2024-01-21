use super::{resource_id_trait::ResourceIdTrait, uid_trait::UidTrait, Note};

pub enum NoteTypes<T, U>
where
    T: ResourceIdTrait,
    U: UidTrait,
{
    Note(Note<U>),
    ResourceRef(T),
}
