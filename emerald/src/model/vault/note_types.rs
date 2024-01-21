use super::{resource_id_trait::ResourceIdTrait, Note};

pub enum NoteTypes<T>
where
    T: ResourceIdTrait + 'static,
{
    Note(Note),
    ResourceRef(T),
}
