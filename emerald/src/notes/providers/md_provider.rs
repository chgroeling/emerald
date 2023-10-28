use crate::types;

pub trait MdProvider {
    fn get_markdown(&self, rid: &types::ResourceId) -> String;
}
