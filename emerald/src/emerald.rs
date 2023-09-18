use crate::indexes::backlink_index::BacklinkIndex;

use crate::content_analyzers::MdLinkAnalyzer;
use crate::indexes::endpoint_index::EndpointIndex;
use crate::indexes::resource_id_index::ResourceIdIndex;
use crate::indexes::AllEndpointsIterSource;
use crate::types::EndPoint;

use crate::resolvers::create_link_resolver;
use crate::resolvers::LinkResolver;
use crate::resources::content_storage::ContentStorage;
use crate::resources::file_content_loader::FileContentLoader;
use crate::Result;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::rc::Rc;
use std::{path::Path, time::Instant};

#[allow(dead_code)]
pub struct Emerald {
    pub md_link_analyzer: Rc<MdLinkAnalyzer>,
    pub endpoint_index: Rc<EndpointIndex>,
    pub resource_id_index: Rc<ResourceIdIndex>,
    pub link_resolver: Rc<dyn LinkResolver>,
    pub backlink_index: Rc<BacklinkIndex>,
    pub content_loader: Rc<FileContentLoader>,
    pub content_storage: Rc<ContentStorage>,
}

impl Emerald {
    pub fn new(vault_path: &Path) -> Result<Emerald> {
        // TODO: ONLY INTEGRATE TIME MEASUREMENT WHEN DEBUG IS ENABLED
        // Build dependency root

        let start = Instant::now();
        let endpoint_index = Rc::new(EndpointIndex::new(vault_path)?);
        let dur = start.elapsed();
        debug!("Creation of EndpointIndex took: {:?}", dur);

        let start = Instant::now();
        let resource_id_index = Rc::new(ResourceIdIndex::new(endpoint_index.as_ref(), vault_path));
        let dur = start.elapsed();
        debug!("Creation of ResourceIdIndex took: {:?}", dur);

        let start = Instant::now();
        let link_resolver = create_link_resolver(resource_id_index.as_ref());
        let dur = start.elapsed();
        debug!("Creation of LinkQueryableImpl took: {:?}", dur);

        let start = Instant::now();
        let md_link_analyzer = Rc::new(MdLinkAnalyzer::new(link_resolver.clone()));
        let dur = start.elapsed();
        debug!("Creation of MdLinkAnalyzer took: {:?}", dur);

        let start = Instant::now();
        let content_loader = Rc::new(FileContentLoader::new(endpoint_index.as_ref(), vault_path));
        let dur = start.elapsed();
        debug!("Creation of ContentLoaderImpl took: {:?}", dur);

        let start = Instant::now();
        let content_storage = Rc::new(ContentStorage::new(
            resource_id_index.as_ref(),
            content_loader.as_ref(),
        ));
        let dur = start.elapsed();
        debug!("Creation of ContentStorage took: {:?}", dur);

        let start = Instant::now();
        let backlink_index = Rc::new(BacklinkIndex::new(
            content_storage.as_ref(),
            md_link_analyzer.as_ref(),
        ));
        let dur = start.elapsed();
        debug!("Creation of BacklinkIndex took: {:?}", dur);

        Ok(Emerald {
            md_link_analyzer,
            link_resolver,
            endpoint_index,
            resource_id_index,
            content_loader,
            content_storage,
            backlink_index,
        })
    }
}
impl Emerald {
    pub fn file_count(&self) -> usize {
        self.endpoint_index.all_iter().count()
    }

    pub fn md_file_count(&self) -> usize {
        self.endpoint_index
            .all_iter()
            .filter(|pred| matches!(pred, EndPoint::FileMarkdown(_)))
            .count()
    }

    pub fn valid_backlink_count(&self) -> usize {
        self.backlink_index.get_valid_backlink_cnt()
    }

    pub fn invalid_backlink_count(&self) -> usize {
        self.backlink_index.get_invalid_backlink_cnt()
    }
}

