use crate::types::ResourceId;

pub trait MdProvider {
    fn get_markdown(&self, resource_id: &ResourceId) -> String;
}
