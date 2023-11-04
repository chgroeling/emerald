use super::links_iter_src::LinksIterSrc;
use super::note_count::NoteCount;
use super::note_meta_data_map::NoteMetaDataMap;
use super::note_meta_data_retriever::NoteMetaDataRetriever;
use super::notes_iter_src::NotesIterSrc;
use super::src_links_map::SrcLinksMap;
use super::tgt_links_map::TgtLinksMap;
use super::TgtIterRetriever;
use crate::types;

#[allow(dead_code)]
pub struct DefaultNoteModel {
    note_index: Vec<types::ResourceId>,
    meta_data_map: NoteMetaDataMap,
    link_index: Vec<types::LinkSrc2Tgt>,
    src_links_map: SrcLinksMap,
    tgt_links_map: TgtLinksMap,
}

impl DefaultNoteModel {
    pub fn new(
        it_note_meta_data: impl IntoIterator<Item = (types::ResourceId, types::MetaData)>,
        it_links_src_2_tgt: impl IntoIterator<Item = types::LinkSrc2Tgt>,
    ) -> DefaultNoteModel {
        let link_index: Vec<_> = it_links_src_2_tgt.into_iter().collect();
        let src_links_map = SrcLinksMap::new(link_index.iter());
        let tgt_links_map = TgtLinksMap::new(link_index.iter());
        let it_note_meta: Vec<_> = it_note_meta_data.into_iter().collect();
        DefaultNoteModel {
            note_index: it_note_meta.iter().map(|f| f.0.clone()).collect(),
            link_index,
            src_links_map,
            tgt_links_map,
            meta_data_map: NoteMetaDataMap::new(it_note_meta),
        }
    }

    pub fn get_links_src_2_tgt_iterator(&self) -> impl Iterator<Item = &types::LinkSrc2Tgt> {
        self.link_index.iter()
    }
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

impl LinksIterSrc for DefaultNoteModel {
    type Iter = std::vec::IntoIter<types::LinkSrc2Tgt>;

    fn create_iter(&self) -> Self::Iter {
        self.link_index.clone().into_iter()
    }
}

impl TgtIterRetriever for DefaultNoteModel {
    fn retrieve(&self, src: &types::ResourceId) -> Option<std::vec::IntoIter<types::Link2Tgt>> {
        self.tgt_links_map.retrieve(src)
    }
}
