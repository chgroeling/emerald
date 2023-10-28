use crate::model::file_model;
use crate::model::note_model;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::vault_file_stats::VaultFileStats;

pub fn extract_file_stats(
    all_res_ids: &impl file_model::FileCount,
    md_res_ids: &impl note_model::NoteCount,
) -> VaultFileStats {
    VaultFileStats {
        file_count: all_res_ids.count(),
        md_file_count: md_res_ids.count(),
    }
}
