use super::note_count::NoteCount;
use super::note_metadata::NoteMetadata;
use super::note_metadata_map::NoteMetadataMap;
use super::note_metadata_retriever::NoteMetadataRetriever;
use super::notes_iter_src::NotesIterSrc;

use crate::types;

pub struct DefaultNoteModel {
    note_index: Vec<types::ResourceId>,
    meta_data_map: NoteMetadataMap,
}

impl DefaultNoteModel {
    pub fn new(
        it_note_meta_data: impl IntoIterator<
            Item = (
                types::ResourceId,
                types::FilesystemMetadata,
                types::DocumentMetadata,
            ),
        >,
    ) -> DefaultNoteModel {
        let it_note_meta: Vec<(_, NoteMetadata)> = it_note_meta_data
            .into_iter()
            .map(|f| (f.0, (f.1, f.2).into()))
            .collect();
        DefaultNoteModel {
            note_index: it_note_meta.iter().map(|f| f.0.clone()).collect(),

            meta_data_map: NoteMetadataMap::new(it_note_meta),
        }
    }
}

impl NoteMetadataRetriever for DefaultNoteModel {
    fn retrieve(&self, md: &types::ResourceId) -> &NoteMetadata {
        // Option is not returned because meta data should be consistent at this point
        self.meta_data_map.retrieve(md)
    }
}

impl NotesIterSrc for DefaultNoteModel {
    type Iter = std::vec::IntoIter<types::ResourceId>;

    fn create_iter(&self) -> Self::Iter {
        self.note_index.clone().into_iter()
    }
}

impl NoteCount for DefaultNoteModel {
    fn count(&self) -> usize {
        self.note_index.len()
    }
}
