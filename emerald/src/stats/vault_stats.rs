use super::{vault_file_stats::VaultFileStats, vault_link_stats::VaultLinkStats};

pub struct VaultStats {
    pub file_stats: VaultFileStats,
    pub link_stats: VaultLinkStats,
}
