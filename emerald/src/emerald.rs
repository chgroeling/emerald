#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::rc::Rc;
use std::{path::Path, time::Instant};

use crate::content_analyzers::MdLinkAnalyzer;
use crate::indexes::endpoint_index::EndpointIndex;
use crate::indexes::resource_id_index::{AllResourceIds, MdResourceIds, ResourceIdIndex};
use crate::indexes::src_2_tgt_index::Src2TargetIndex;
use crate::indexes::EndpointsIterSrc;
use crate::maps::endpoint_resource_id_map::EndpointResourceIdMap;
use crate::maps::resource_id_queryable::ResourceIdQuerier;
use crate::maps::LinkQuerier;
use crate::maps::TgtIterQuerier;
use crate::maps::{create_link_queryable, SrcIterQuerier};
use crate::maps::{create_src_iter_queryable, create_tgt_iter_queryable};
use crate::notes::providers::std_provider_factory::StdProviderFactory;
use crate::notes::vault::Vault;
use crate::resources::content_full_cache::ContentFullCache;
use crate::resources::file_content_loader::FileContentLoader;
use crate::resources::file_meta_data_loader::FileMetaDataLoader;
use crate::resources::meta_data_loader::MetaDataLoader;
use crate::types::EndPoint;
use crate::Result;

#[allow(dead_code)]
pub struct Emerald {
    pub md_link_analyzer: Rc<MdLinkAnalyzer>,
    pub ep_index: Rc<EndpointIndex>,
    pub resource_id_queryable: Rc<dyn ResourceIdQuerier>,
    pub meta_data_loader: Rc<dyn MetaDataLoader>,
    pub resource_id_index: Rc<ResourceIdIndex>,
    pub link_queryable: Rc<dyn LinkQuerier>,
    pub tgt_iter_queryable: Rc<dyn TgtIterQuerier>,
    pub src_iter_queryable: Rc<dyn SrcIterQuerier>,
    pub note_link_index: Rc<Src2TargetIndex>,
    pub content_loader: Rc<FileContentLoader>,
    pub content_storage: Rc<ContentFullCache>,
    pub std_provider_factory: Rc<StdProviderFactory<FileMetaDataLoader, ContentFullCache>>,
    pub vault: Rc<Vault<MdResourceIds>>,
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
        let meta_data_loader = Rc::new(FileMetaDataLoader::new(resource_id_queryable.clone()));
        debug!("Creation of FileMetaDataLoader took: {:?}", start.elapsed());

        let start = Instant::now();
        let resource_id_index = Rc::new(ResourceIdIndex::new(ep_index.as_ref(), vault_path));
        let all_res_ids_iter_rc = Rc::new(AllResourceIds::new_from_rc(&resource_id_index));
        let md_res_ids_iter_rc = Rc::new(MdResourceIds::new_from_rc(&resource_id_index));
        debug!("Creation of ResourceIdIndex took: {:?}", start.elapsed());

        let start = Instant::now();
        let link_queryable = create_link_queryable(all_res_ids_iter_rc.as_ref());
        debug!("Creation of LinkQuerierImpl took: {:?}", start.elapsed());

        let start = Instant::now();
        let md_link_analyzer = Rc::new(MdLinkAnalyzer::new(link_queryable.clone()));
        debug!("Creation of MdLinkAnalyzer took: {:?}", start.elapsed());

        let start = Instant::now();
        let content_loader = Rc::new(FileContentLoader::new(resource_id_queryable.clone()));
        debug!("Creation of FileContentLoader took: {:?}", start.elapsed());

        let start = Instant::now();
        let content_storage = Rc::new(ContentFullCache::new(
            md_res_ids_iter_rc.as_ref(),
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
        let tgt_iter_queryable = create_tgt_iter_queryable(note_link_index.as_ref());
        debug!("Creation of TgtIterQuerier took: {:?}", start.elapsed());

        let start = Instant::now();
        let src_iter_queryable = create_src_iter_queryable(note_link_index.as_ref());
        debug!("Creation of SrcIterQuerier took: {:?}", start.elapsed());

        let start = Instant::now();
        let std_provider_factory = Rc::new(StdProviderFactory::new(
            meta_data_loader.clone(),
            content_storage.clone(),
        ));
        debug!("Creation of StdProviderFactory took: {:?}", start.elapsed());

        let start = Instant::now();
        let vault = Rc::new(Vault::new(
            md_res_ids_iter_rc.clone(),
            std_provider_factory.clone(),
        ));
        debug!("Creation of Vault took: {:?}", start.elapsed());

        Ok(Emerald {
            md_link_analyzer,
            resource_id_queryable,
            meta_data_loader,
            link_queryable,
            ep_index,
            resource_id_index,
            content_loader,
            content_storage,
            note_link_index,
            tgt_iter_queryable,
            src_iter_queryable,
            std_provider_factory,
            vault,
        })
    }
}

impl Emerald {
    pub fn get_vault(&self) -> Rc<Vault<MdResourceIds>> {
        self.vault.clone()
    }

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
