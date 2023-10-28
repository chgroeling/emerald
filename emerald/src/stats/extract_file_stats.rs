use crate::model;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::vault_file_stats::VaultFileStats;

pub fn extract_file_stats(
    all_res_ids: &impl model::FileCount,
    md_res_ids: &impl model::NoteCount,
) -> VaultFileStats {
    VaultFileStats {
        file_count: all_res_ids.count(),
        md_file_count: md_res_ids.count(),
    }
}
