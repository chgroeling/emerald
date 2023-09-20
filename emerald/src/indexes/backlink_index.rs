#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::{
    content_analyzers::MdLinkAnalyzerIterSource, resources::ContentIterSource, types::NoteLink,
};

use super::all_note_links_iter_source::AllNoteLinksIterSource;

#[allow(dead_code)]
pub type NoteLinkList = Vec<NoteLink>;

pub struct BacklinkIndex {
    valid_backlink_cnt: usize,
    invalid_backlink_cnt: usize,

    #[allow(dead_code)]
    note_link_list: NoteLinkList,
}

impl BacklinkIndex {
    pub fn new(
        content_iter_src: &impl ContentIterSource,
        md_link_analyzer: &impl MdLinkAnalyzerIterSource,
    ) -> Self {
        let mut valid_backlink_cnt: usize = 0;
        let mut invalid_backlink_cnt: usize = 0;
        let mut note_link_list = NoteLinkList::new();

        for (dest, content) in content_iter_src.iter() {
            trace!("Link extraction from {:?} starts", &dest);

            let mut note_valid_backlink_cnt: usize = 0;
            let mut note_invalid_backlink_cnt: usize = 0;
            for link_and_resource_id in md_link_analyzer.create_iter(content.0.as_ref().clone()) {
                match &link_and_resource_id {
                    (link, None) => {
                        note_invalid_backlink_cnt += 1;
                        warn!("Parsing {:?} -> Link not found: {:?}", &dest, &link);
                    }
                    _ => note_valid_backlink_cnt += 1,
                }
                let note_link = NoteLink {
                    origin: dest.clone(),
                    link: link_and_resource_id.0,
                    dest: link_and_resource_id.1,
                };
                note_link_list.push(note_link);
            }
            if note_valid_backlink_cnt == 0 {
                trace!("No valid links found in  {:?}", &dest);
            }

            valid_backlink_cnt += note_valid_backlink_cnt;
            invalid_backlink_cnt += note_invalid_backlink_cnt;
        }

        Self {
            valid_backlink_cnt,
            invalid_backlink_cnt,
            note_link_list,
        }
    }

    pub fn get_valid_backlink_cnt(&self) -> usize {
        self.valid_backlink_cnt
    }

    pub fn get_invalid_backlink_cnt(&self) -> usize {
        self.invalid_backlink_cnt
    }
}

impl AllNoteLinksIterSource for BacklinkIndex {
    type Iter = std::vec::IntoIter<NoteLink>;
    fn all_iter(&self) -> Self::Iter {
        self.note_link_list.clone().into_iter()
    }
}
