use crate::types;

pub trait TimestampProvider {
    fn get_timestamp(&self, rid: &types::ResourceId) -> i64;
}
