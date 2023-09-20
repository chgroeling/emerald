#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::{
    content_analyzers::MdLinkAnalyzerIterSource, resources::ContentIterSource, types::NoteLink,
};

#[allow(dead_code)]
pub type ResourceIdToBacklinks = Vec<NoteLink>;

pub struct BacklinkIndex {
    valid_backlink_cnt: usize,
    invalid_backlink_cnt: usize,

    #[allow(dead_code)]
    resource_id_to_backlinks: ResourceIdToBacklinks,
}

impl BacklinkIndex {
    pub fn new(
        content_iter_src: &impl ContentIterSource,
        link_extractor: &impl MdLinkAnalyzerIterSource,
    ) -> Self {
        let mut valid_backlink_cnt: usize = 0;
        let mut invalid_backlink_cnt: usize = 0;
        let mut resource_id_to_backlinks = ResourceIdToBacklinks::new();

        for (dest, content) in content_iter_src.iter() {
            trace!("Link extraction from {:?} starts", &dest);

            let mut note_valid_backlink_cnt: usize = 0;
            let mut note_invalid_backlink_cnt: usize = 0;
            for link_and_resource_id in link_extractor.create_iter(content.0.as_ref().clone()) {
                match &link_and_resource_id {
                    (link, None) => {
                        note_invalid_backlink_cnt += 1;
                        warn!("Parsing {:?} -> Link not found: {:?}", &dest, &link);
                    }
                    _ => note_valid_backlink_cnt += 1,
                }
                let bindex = NoteLink {
                    origin: dest.clone(),
                    link: link_and_resource_id.0,
                    dest: link_and_resource_id.1,
                };
                resource_id_to_backlinks.push(bindex);
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
            resource_id_to_backlinks,
        }
    }

    pub fn get_valid_backlink_cnt(&self) -> usize {
        self.valid_backlink_cnt
    }

    pub fn get_invalid_backlink_cnt(&self) -> usize {
        self.invalid_backlink_cnt
    }
}
