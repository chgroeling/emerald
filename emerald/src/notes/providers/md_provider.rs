use crate::types;

pub trait MdProvider {
    fn get_markdown(&self, resource_id: &types::ResourceId) -> String;
}
