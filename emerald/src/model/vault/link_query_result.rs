use super::resource_id_trait::ResourceIdTrait;

pub enum LinkQueryResult<T>
where
    T: ResourceIdTrait,
{
    LinkToNote(T),
    LinkToResource(T),
}
