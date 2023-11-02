use crate::types;

pub trait Provider<T> {
    fn get(&self, rid: &types::ResourceId) -> T;
}
