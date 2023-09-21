#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::{
    content_analyzers::MdLinkAnalyzerIterSource, resources::ContentIterSource,
    types::LinkOriginDestination,
};

use super::all_note_links_iter_source::AllNoteLinksIterSource;

#[allow(dead_code)]
pub type LinkOriginDestinationList = Vec<LinkOriginDestination>;

pub struct NoteLinkIndex {
    valid_backlink_cnt: usize,
    invalid_backlink_cnt: usize,

    #[allow(dead_code)]
    link_origin_dest_list: LinkOriginDestinationList,
}

impl NoteLinkIndex {
    pub fn new(
        content_iter_src: &impl ContentIterSource,
        md_link_analyzer: &impl MdLinkAnalyzerIterSource,
    ) -> Self {
        let mut valid_backlink_cnt: usize = 0;
        let mut invalid_backlink_cnt: usize = 0;
        let mut link_origin_dest_list = LinkOriginDestinationList::new();

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
                let note_link = LinkOriginDestination {
                    origin: dest.clone(),
                    link_and_destination: link_and_resource_id,
                };
                link_origin_dest_list.push(note_link);
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
            link_origin_dest_list,
        }
    }

    pub fn get_valid_backlink_cnt(&self) -> usize {
        self.valid_backlink_cnt
    }

    pub fn get_invalid_backlink_cnt(&self) -> usize {
        self.invalid_backlink_cnt
    }
}

impl AllNoteLinksIterSource for NoteLinkIndex {
    type Iter = std::vec::IntoIter<LinkOriginDestination>;
    fn all_iter(&self) -> Self::Iter {
        self.link_origin_dest_list.clone().into_iter()
    }
}
