use crate::types;

pub trait TimestampProvider {
    fn get(&self, rid: &types::ResourceId) -> i64;
}
