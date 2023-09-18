use std::collections::HashMap;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::{
    content_analyzers::MdLinkAnalyzerIterSource,
    resources::ContentIterSource,
    types::{LinkAndResourceId, ResourceId},
};

//TODO: Include iterator traits

pub type ResourceIdToBacklinks = HashMap<ResourceId, Vec<LinkAndResourceId>>;

pub struct BacklinkIndex {
    valid_backlink_cnt: usize,
    invalid_backlink_cnt: usize,

    #[allow(dead_code)]
    resource_id_to_backlinks: ResourceIdToBacklinks,
}

impl<'a> BacklinkIndex {
    pub fn new(
        content_iter_src: &'a impl ContentIterSource<'a>,
        link_extractor: &'a impl MdLinkAnalyzerIterSource,
    ) -> Self {
        let mut valid_backlink_cnt: usize = 0;
        let mut invalid_backlink_cnt: usize = 0;
        let mut resource_id_to_backlinks = ResourceIdToBacklinks::new();

        for (resource_id, content) in content_iter_src.iter() {
            trace!("Link extraction from {:?} starts", &resource_id);

            let mut resource_id_backlinks: Vec<LinkAndResourceId> = Vec::new();

            for link_and_resource_id in link_extractor.create_iter(content.0.as_ref().clone()) {
                match &link_and_resource_id {
                    (link, None) => {
                        invalid_backlink_cnt += 1;
                        warn!("Parsing {:?} -> Link not found: {:?}", &resource_id, &link);
                    }
                    _ => valid_backlink_cnt += 1,
                }
                resource_id_backlinks.push(link_and_resource_id);
            }
            if resource_id_backlinks.is_empty() {
                trace!("No valid links found in  {:?}", &resource_id);
            }

            resource_id_to_backlinks.insert(resource_id.clone(), resource_id_backlinks);
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
