use crate::types;

pub trait StringProvider {
    fn get_string(&self, rid: &types::ResourceId) -> String;
}
