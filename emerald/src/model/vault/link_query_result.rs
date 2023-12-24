use crate::types;

pub enum LinkQueryResult {
    LinkToNote(types::ResourceId),
    LinkToResource(types::ResourceId),
}
