use super::ExResourceId;

pub enum LinkQueryResult {
    LinkToNote(ExResourceId),
    LinkToResource(ExResourceId),
}
