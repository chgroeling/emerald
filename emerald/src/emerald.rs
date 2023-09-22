#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::rc::Rc;
use std::{path::Path, time::Instant};

use crate::content_analyzers::MdLinkAnalyzer;
use crate::indexes::endpoint_index::EndpointIndex;
use crate::indexes::resource_id_index::ResourceIdIndex;
use crate::indexes::source_to_target_index::LinkFromSourceToTargetIndex;
use crate::indexes::AllEndpointsIterable;
use crate::maps::LinkQueryable;
use crate::maps::TargetIteratorQueryable;
use crate::maps::{create_link_queryable, SourceIteratorQueryable};
use crate::maps::{create_source_iterator_queryable, create_target_iterator_queryable};
use crate::resources::content_storage::ContentStorage;
use crate::resources::file_content_loader::FileContentLoader;
use crate::types::EndPoint;
use crate::Result;

#[allow(dead_code)]
pub struct Emerald {
    pub md_link_analyzer: Rc<MdLinkAnalyzer>,
    pub endpoint_index: Rc<EndpointIndex>,
    pub resource_id_index: Rc<ResourceIdIndex>,
    pub link_queryable: Rc<dyn LinkQueryable>,
    pub target_iterator_queryable: Rc<dyn TargetIteratorQueryable>,
    pub source_iterator_queryable: Rc<dyn SourceIteratorQueryable>,
    pub note_link_index: Rc<LinkFromSourceToTargetIndex>,
    pub content_loader: Rc<FileContentLoader>,
    pub content_storage: Rc<ContentStorage>,
}

impl Emerald {
    pub fn new(vault_path: &Path) -> Result<Emerald> {
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
        let link_resolver = create_link_queryable(resource_id_index.as_ref());
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
        let note_link_index = Rc::new(LinkFromSourceToTargetIndex::new(
            content_storage.as_ref(),
            md_link_analyzer.as_ref(),
        ));
        let dur = start.elapsed();
        debug!("Creation of NoteLinkIndex took: {:?}", dur);

        let start = Instant::now();
        let target_iterator_queryable = create_target_iterator_queryable(note_link_index.as_ref());
        let dur = start.elapsed();
        debug!("Creation of TargetIteratorQueryable took: {:?}", dur);

        let start = Instant::now();
        let source_iterator_queryable = create_source_iterator_queryable(note_link_index.as_ref());
        let dur = start.elapsed();
        debug!("Creation of SourceIteratorQueryable took: {:?}", dur);

        Ok(Emerald {
            md_link_analyzer,
            link_queryable: link_resolver,
            endpoint_index,
            resource_id_index,
            content_loader,
            content_storage,
            note_link_index,
            target_iterator_queryable,
            source_iterator_queryable,
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
        self.note_link_index.get_valid_backlink_cnt()
    }

    pub fn invalid_backlink_count(&self) -> usize {
        self.note_link_index.get_invalid_backlink_cnt()
    }
}
