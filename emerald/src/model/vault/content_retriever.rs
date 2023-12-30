use super::VaultResourceId;

pub trait ContentRetriever {
    fn retrieve(&self, rid: &VaultResourceId) -> &str;
}
