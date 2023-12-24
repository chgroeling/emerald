use super::ResourceId;

pub enum LinkQueryResult {
    LinkToNote(ResourceId),
    LinkToResource(ResourceId),
}
