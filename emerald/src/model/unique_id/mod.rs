pub mod resource_id_trait;
pub mod uid;
pub mod uid_map;

use std::rc::Rc;

pub use uid::Uid;
pub use uid_map::UidMap;

#[derive(Clone)]
pub struct UniqueId<T>
where
    T: resource_id_trait::ResourceIdTrait,
{
    uid_map: Rc<UidMap<T>>,
}

impl<T> UniqueId<T>
where
    T: resource_id_trait::ResourceIdTrait,
{
    pub fn new(note_rid_iter: impl IntoIterator<Item = T>) -> Self {
        let mut uid_map = UidMap::<T>::new();

        for rid in note_rid_iter.into_iter() {
            uid_map.assign_uid(&rid);
        }

        Self {
            uid_map: Rc::new(uid_map),
        }
    }
}
