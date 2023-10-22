use crate::types;

pub trait TitleProvider {
    fn get_title(&self, resource_id: &types::ResourceId) -> String;
}
