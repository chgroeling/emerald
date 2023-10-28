use crate::{
    model::{FilesIterSrc, NotesIterSrc},
    types,
};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::vault_file_stats::VaultFileStats;

pub fn extract_file_stats<'a>(
    all_res_ids: &impl FilesIterSrc,
    md_res_ids: &impl NotesIterSrc,
) -> VaultFileStats {
    VaultFileStats {
        file_count: all_res_ids.create_iter().count(),
        md_file_count: md_res_ids.create_iter().count(),
    }
}
