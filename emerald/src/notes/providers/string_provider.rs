use crate::types;

pub trait StringProvider {
    fn get(&self, rid: &types::ResourceId) -> String;
}
