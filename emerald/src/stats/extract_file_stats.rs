use super::vault_file_stats::VaultFileStats;
use crate::model::file;
use crate::model::note;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn extract_file_stats(
    all_res_ids: &impl file::FileCount,
    md_res_ids: &impl note::NoteCount,
) -> VaultFileStats {
    VaultFileStats {
        file_count: all_res_ids.count(),
        md_file_count: md_res_ids.count(),
    }
}
