use crate::types;

use super::note_metadata::NoteMetadata;

/// This trait is used to query an target id for all contained links and their pointing resource ids.
pub trait NoteMetadataRetriever {
    fn retrieve(&self, tgt: &types::ResourceId) -> &NoteMetadata;
}
