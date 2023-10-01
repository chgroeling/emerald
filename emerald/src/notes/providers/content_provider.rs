use crate::types::ResourceId;

pub trait ContentProvider {
    fn get_content(&self, resource_id: &ResourceId) -> String;
}
