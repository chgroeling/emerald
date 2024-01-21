use crate::model::unique_id;

use super::{resource_id_trait::ResourceIdTrait, Note};

pub enum NoteTypes<T>
where
    T: ResourceIdTrait,
{
    Note(Note<unique_id::Uid>),
    ResourceRef(T),
}
