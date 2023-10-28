use crate::types;

pub trait TitleProvider {
    fn get_title(&self, rid: &types::ResourceId) -> String;
}
