use crate::types::ResourceId;

pub trait ContentIterSource<'a> {
    // TODO: Should this be (&ResourceId, &String)?
    type Iter: Iterator<Item = &'a (ResourceId, String)>;
    fn iter(&'a self) -> Self::Iter;
}
