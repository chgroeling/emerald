use crate::types;

use super::note_meta_data::NoteMetaData;

/// This trait is used to query an target id for all contained links and their pointing resource ids.
pub trait NoteMetaDataRetriever {
    fn retrieve(&self, tgt: &types::ResourceId) -> &NoteMetaData;
}
