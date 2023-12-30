use super::VaultResourceId;

pub enum LinkQueryResult {
    LinkToNote(VaultResourceId),
    LinkToResource(VaultResourceId),
}
