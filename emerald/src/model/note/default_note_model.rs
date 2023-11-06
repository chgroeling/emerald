use super::note_count::NoteCount;
use super::note_meta_data_map::NoteMetaDataMap;
use super::note_meta_data_retriever::NoteMetaDataRetriever;
use super::notes_iter_src::NotesIterSrc;

use crate::types;

#[allow(dead_code)]
pub struct DefaultNoteModel {
    note_index: Vec<types::ResourceId>,
    meta_data_map: NoteMetaDataMap,
}

impl DefaultNoteModel {
    pub fn new(
        it_note_meta_data: impl IntoIterator<Item = (types::ResourceId, types::MetaData)>,
    ) -> DefaultNoteModel {
        let it_note_meta: Vec<_> = it_note_meta_data.into_iter().collect();
        DefaultNoteModel {
            note_index: it_note_meta.iter().map(|f| f.0.clone()).collect(),

            meta_data_map: NoteMetaDataMap::new(it_note_meta),
        }
    }

    /*  pub fn get_links_src_2_tgt_iterator(&self) -> impl Iterator<Item = &types::LinkSrc2Tgt> {
        self.link_index.iter()
    }*/
}

impl NoteMetaDataRetriever for DefaultNoteModel {
    fn retrieve(&self, md: &types::ResourceId) -> &types::MetaData {
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
