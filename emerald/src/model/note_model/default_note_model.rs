use crate::types;

use super::meta_data_map::MetaDataMap;
use super::meta_data_retriever::MetaDataRetriever;
use super::notes_iter_src::NotesIterSrc;
use super::src_links_map::SrcLinksMap;
use super::tgt_links_map::TgtLinksMap;

#[allow(dead_code)]
pub struct DefaultNoteModel {
    note_index: Vec<types::ResourceId>,
    all_links: Vec<types::LinkSrc2Tgt>,
    src_links_map: SrcLinksMap,
    tgt_links_map: TgtLinksMap,
    meta_data_map: MetaDataMap,
}

impl DefaultNoteModel {
    pub fn new<'a>(
        it_note_meta_data: impl IntoIterator<Item = (&'a types::ResourceId, &'a types::MetaData)>
            + Clone,
        it_links_src_2_tgt: impl IntoIterator<Item = &'a types::LinkSrc2Tgt>,
    ) -> DefaultNoteModel {
        let all_links: Vec<_> = it_links_src_2_tgt.into_iter().cloned().collect();
        let src_links_map = SrcLinksMap::new(all_links.iter());
        let tgt_links_map = TgtLinksMap::new(all_links.iter());
        DefaultNoteModel {
            note_index: it_note_meta_data
                .clone()
                .into_iter()
                .map(|f| f.0)
                .cloned()
                .collect(),
            all_links,
            src_links_map,
            tgt_links_map,
            meta_data_map: MetaDataMap::new(it_note_meta_data),
        }
    }

    pub fn get_links_src_2_tgt_iterator(&self) -> impl Iterator<Item = &types::LinkSrc2Tgt> {
        self.all_links.iter()
    }
}

impl MetaDataRetriever for DefaultNoteModel {
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
