use crate::types::ResourceId;

pub trait TitleProvider {
    fn get_title(&self, resource_id: &ResourceId) -> String;
}
