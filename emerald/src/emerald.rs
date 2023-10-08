#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::rc::Rc;
use std::{path::Path, time::Instant};

use crate::content_analyzers::MdLinkAnalyzer;
use crate::indexes::resource_id_converter::ResourceIdConverter;
use crate::indexes::resource_id_index::{AllResourceIds, MdResourceIds, ResourceIdIndex};
use crate::indexes::src_2_tgt_index::Src2TargetIndex;
use crate::maps::ResourceIdRetriever;
use crate::maps::TgtIterRetriever;
use crate::maps::{create_resource_id_retriever, SrcIterRetriever};
use crate::maps::{create_src_iter_retriever, create_tgt_iter_retriever};
use crate::notes::providers::std_provider_factory::StdProviderFactory;
use crate::notes::vault::Vault;
use crate::resources::content_full_md_cache::ContentFullMdCache;
use crate::resources::endpoint_index::EndpointIndex;
use crate::resources::endpoint_resource_id_map::EndpointResourceIdMap;
use crate::resources::endpoints_iter_src::EndpointsIterSrc;
use crate::resources::file_content_loader::FileContentLoader;
use crate::resources::file_meta_data_loader::FileMetaDataLoader;
use crate::resources::resource_id_endpoint_map::ResourceIdEndPointMap;
use crate::types::EndPoint;
use crate::Result;

type FileMetaDataLoaderImpl = FileMetaDataLoader<EndpointResourceIdMap>;
type ResourceIdIndexImpl = ResourceIdIndex<FileMetaDataLoaderImpl>;
type MdResourceIdsImpl = MdResourceIds<FileMetaDataLoaderImpl>;

#[allow(dead_code)]
pub struct Emerald {
    pub ep_iter_src: EndpointIndex,
    pub resource_id_resolver: ResourceIdEndPointMap,
    pub endpoint_resolver: EndpointResourceIdMap,
    pub meta_data_loader: FileMetaDataLoaderImpl,
    pub resource_id_index: ResourceIdIndexImpl,
    pub md_link_analyzer: Rc<MdLinkAnalyzer>,
    pub resource_id_retriever: Rc<dyn ResourceIdRetriever>,
    pub tgt_iter_retriever: Rc<dyn TgtIterRetriever>,
    pub src_iter_retriever: Rc<dyn SrcIterRetriever>,
    pub note_link_index: Rc<Src2TargetIndex>,
    pub content_loader: Rc<FileContentLoader<EndpointResourceIdMap>>,
    pub content_full_md_cache: Rc<ContentFullMdCache<FileContentLoader<EndpointResourceIdMap>>>,
    pub std_provider_factory: Rc<
        StdProviderFactory<
            FileMetaDataLoader<EndpointResourceIdMap>,
            ContentFullMdCache<FileContentLoader<EndpointResourceIdMap>>,
        >,
    >,
    pub vault: Rc<Vault<MdResourceIdsImpl>>,
}

impl Emerald {
    pub fn new(vault_path: &Path) -> Result<Emerald> {
        // Build dependency root
        let start = Instant::now();
        let ep_iter_src = EndpointIndex::new(vault_path)?;
        debug!("Creation of EndpointIndex took: {:?}", start.elapsed());

        let start = Instant::now();
        let resource_id_resolver = ResourceIdEndPointMap::new(&ep_iter_src, vault_path);
        debug!(
            "Creation of ResourceIdEndPointMap took: {:?}",
            start.elapsed()
        );

        let start = Instant::now();
        let endpoint_resolver = EndpointResourceIdMap::new(&ep_iter_src, &resource_id_resolver);
        debug!(
            "Creation of EndpointResourceIdMap took: {:?}",
            start.elapsed()
        );

        let start = Instant::now();
        let meta_data_loader = FileMetaDataLoader::new(endpoint_resolver.clone());
        debug!("Creation of FileMetaDataLoader took: {:?}", start.elapsed());

        let start = Instant::now();
        let resource_id_iter_src_not_cached = Rc::new(ResourceIdConverter {
            ep_iter_src: ep_iter_src.clone(),
            resource_id_resolver: resource_id_resolver.clone(),
        });

        let mut resource_id_index = ResourceIdIndex::new(meta_data_loader.clone());
        resource_id_index.update(resource_id_iter_src_not_cached.as_ref());
        let all_res_ids_iter_rc = AllResourceIds::new(resource_id_index.clone());
        let md_res_ids_iter_rc = MdResourceIds::new(resource_id_index.clone());
        debug!("Creation of ResourceIdIndex took: {:?}", start.elapsed());

        let start = Instant::now();
        let resource_id_retriever = create_resource_id_retriever(&all_res_ids_iter_rc);
        debug!(
            "Creation of ResourceIdRetriever took: {:?}",
            start.elapsed()
        );

        let start = Instant::now();
        let md_link_analyzer = Rc::new(MdLinkAnalyzer::new(resource_id_retriever.clone()));
        debug!("Creation of MdLinkAnalyzer took: {:?}", start.elapsed());

        let start = Instant::now();
        let content_loader = Rc::new(FileContentLoader::new(endpoint_resolver.clone()));
        debug!("Creation of FileContentLoader took: {:?}", start.elapsed());

        let start = Instant::now();
        let content_full_md_cache = Rc::new(ContentFullMdCache::new(
            &md_res_ids_iter_rc,
            content_loader.clone(),
        ));
        debug!("Creation of ContentFullMdCache took: {:?}", start.elapsed());

        let start = Instant::now();
        let note_link_index = Rc::new(Src2TargetIndex::new(
            content_full_md_cache.as_ref(),
            &md_res_ids_iter_rc,
            md_link_analyzer.as_ref(),
        ));
        debug!("Creation of Src2TargetIndex took: {:?}", start.elapsed());

        let start = Instant::now();
        let tgt_iter_retriever = create_tgt_iter_retriever(note_link_index.as_ref());
        debug!("Creation of TgtIterRetriever took: {:?}", start.elapsed());

        let start = Instant::now();
        let src_iter_retriever = create_src_iter_retriever(note_link_index.as_ref());
        debug!("Creation of SrcIterRetriever took: {:?}", start.elapsed());

        let start = Instant::now();
        let std_provider_factory = Rc::new(StdProviderFactory::new(
            meta_data_loader.clone(),
            content_full_md_cache.clone(),
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
            resource_id_resolver,
            endpoint_resolver,
            meta_data_loader,
            resource_id_retriever,
            ep_iter_src,
            resource_id_index,
            content_loader,
            content_full_md_cache,
            note_link_index,
            tgt_iter_retriever,
            src_iter_retriever,
            std_provider_factory,
            vault,
        })
    }
}

impl Emerald {
    pub fn get_vault(&self) -> Rc<Vault<MdResourceIdsImpl>> {
        self.vault.clone()
    }

    pub fn file_count(&self) -> usize {
        self.ep_iter_src.iter().count()
    }

    pub fn md_file_count(&self) -> usize {
        self.ep_iter_src
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
