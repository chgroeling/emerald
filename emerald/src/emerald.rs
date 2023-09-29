#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::rc::Rc;
use std::{path::Path, time::Instant};

use crate::content_analyzers::MdLinkAnalyzer;
use crate::indexes::endpoint_index::EndpointIndex;
use crate::indexes::resource_id_index::{AllResourceIds, MdResourceIds, ResourceIdIndex};
use crate::indexes::src_2_tgt_index::Src2TargetIndex;
use crate::indexes::EndpointsIterable;
use crate::maps::endpoint_resource_id_map::EndpointResourceIdMap;
use crate::maps::resource_id_queryable::ResourceIdQueryable;
use crate::maps::LinkQueryable;
use crate::maps::TgtIterQueryable;
use crate::maps::{create_link_queryable, SrcIterQueryable};
use crate::maps::{create_src_iter_queryable, create_tgt_iter_queryable};
use crate::resources::content_storage::ContentStorage;
use crate::resources::file_content_loader::FileContentLoader;
use crate::types::EndPoint;
use crate::Result;

#[allow(dead_code)]
pub struct Emerald {
    pub md_link_analyzer: Rc<MdLinkAnalyzer>,
    pub ep_index: Rc<EndpointIndex>,
    pub resource_id_queryable: Rc<dyn ResourceIdQueryable>,
    pub resource_id_index: Rc<ResourceIdIndex>,
    pub link_queryable: Rc<dyn LinkQueryable>,
    pub target_iterator_queryable: Rc<dyn TgtIterQueryable>,
    pub source_iterator_queryable: Rc<dyn SrcIterQueryable>,
    pub note_link_index: Rc<Src2TargetIndex>,
    pub content_loader: Rc<FileContentLoader>,
    pub content_storage: Rc<ContentStorage>,
}

impl Emerald {
    pub fn new(vault_path: &Path) -> Result<Emerald> {
        // Build dependency root
        let start = Instant::now();
        let ep_index = Rc::new(EndpointIndex::new(vault_path)?);
        debug!("Creation of EndpointIndex took: {:?}", start.elapsed());

        let start = Instant::now();
        let resource_id_queryable =
            Rc::new(EndpointResourceIdMap::new(ep_index.as_ref(), vault_path));
        debug!(
            "Creation of EndpointResourceIdMap took: {:?}",
            start.elapsed()
        );

        let start = Instant::now();
        let resource_id_index = Rc::new(ResourceIdIndex::new(ep_index.as_ref(), vault_path));
        let all_res_ids_iterable = Rc::new(AllResourceIds::new_from_rc(&resource_id_index));
        let md_res_ids_iterable = Rc::new(MdResourceIds::new_from_rc(&resource_id_index));
        debug!("Creation of ResourceIdIndex took: {:?}", start.elapsed());

        let start = Instant::now();
        let link_queryable = create_link_queryable(all_res_ids_iterable.as_ref());
        debug!("Creation of LinkQueryableImpl took: {:?}", start.elapsed());

        let start = Instant::now();
        let md_link_analyzer = Rc::new(MdLinkAnalyzer::new(link_queryable.clone()));
        debug!("Creation of MdLinkAnalyzer took: {:?}", start.elapsed());

        let start = Instant::now();
        let content_loader = Rc::new(FileContentLoader::new(resource_id_queryable.clone()));
        debug!("Creation of FileContentLoader took: {:?}", start.elapsed());

        let start = Instant::now();
        let content_storage = Rc::new(ContentStorage::new(
            md_res_ids_iterable.as_ref(),
            content_loader.as_ref(),
        ));
        debug!("Creation of ContentStorage took: {:?}", start.elapsed());

        let start = Instant::now();
        let note_link_index = Rc::new(Src2TargetIndex::new(
            content_storage.as_ref(),
            md_link_analyzer.as_ref(),
        ));
        debug!(
            "Creation of LinkFrmSrcToTargetIndex took: {:?}",
            start.elapsed()
        );

        let start = Instant::now();
        let target_iterator_queryable = create_tgt_iter_queryable(note_link_index.as_ref());
        debug!(
            "Creation of TargetIteratorQueryable took: {:?}",
            start.elapsed()
        );

        let start = Instant::now();
        let source_iterator_queryable = create_src_iter_queryable(note_link_index.as_ref());
        debug!(
            "Creation of SourceIteratorQueryable took: {:?}",
            start.elapsed()
        );

        Ok(Emerald {
            md_link_analyzer,
            resource_id_queryable,
            link_queryable,
            ep_index,
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
        self.ep_index.iter().count()
    }

    pub fn md_file_count(&self) -> usize {
        self.ep_index
            .iter()
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
