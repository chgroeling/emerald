use crate::indexes::Src2TargetIndex;
use crate::maps::resource_id_link_map::ResourceIdLinkMap;
use crate::maps::src_links_map::SrcLinksMap;
use crate::maps::tgt_links_map::TgtLinksMap;
use crate::md_analyzer::analyze_markdown;
use crate::notes::providers::std_provider_factory::StdProviderFactory;
use crate::notes::vault::Vault;
use crate::resources::content_full_md_cache::ContentFullMdCache;
use crate::resources::endpoint_index::EndpointIndex;
use crate::resources::endpoint_resource_id_map::EndpointResourceIdMap;
use crate::resources::endpoints_iter_src::EndpointsIterSrc;
use crate::resources::file_content_loader::FileContentLoader;
use crate::resources::file_meta_data_loader::FileMetaDataLoader;
use crate::resources::resource_id_endpoint_map::ResourceIdEndPointMap;
use crate::trafos::{
    extract_links_from_vault, filter_markdown_types, trafo_ep_to_rid,
    trafo_to_filetype_and_resource_id,
};
use crate::types::{EndPoint, ResourceId};
use crate::Result;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{path::Path, time::Instant};

type FileMetaDataLoaderImpl = FileMetaDataLoader<EndpointResourceIdMap>;
type ContentFullMdCacheImpl = ContentFullMdCache<FileContentLoader<EndpointResourceIdMap>>;
type StdProviderFactoryImpl = StdProviderFactory<FileMetaDataLoaderImpl, ContentFullMdCacheImpl>;

#[allow(dead_code)]
pub struct Emerald {
    pub ep_iter_src: EndpointIndex,
    pub resource_id_resolver: ResourceIdEndPointMap,
    pub endpoint_resolver: EndpointResourceIdMap,
    pub meta_data_loader: FileMetaDataLoaderImpl,
    pub resource_id_retriever: ResourceIdLinkMap,
    pub src_2_tgt_iter_src: Src2TargetIndex,
    pub content_cache: ContentFullMdCacheImpl,
    pub tgt_iter_retriever: TgtLinksMap,
    pub src_iter_retriever: SrcLinksMap,
    pub provider_factory: StdProviderFactoryImpl,
    pub vault: Vault<StdProviderFactoryImpl>,
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
        let content_loader = FileContentLoader::new(endpoint_resolver.clone());
        debug!("Creation of FileContentLoader took: {:?}", start.elapsed());

        let start = Instant::now();
        let meta_data_loader = FileMetaDataLoader::new(endpoint_resolver.clone());
        debug!("Creation of FileMetaDataLoader took: {:?}", start.elapsed());

        let start = Instant::now();

        let res_id_iter = trafo_ep_to_rid(ep_iter_src.iter(), &resource_id_resolver);
        let all_resource_ids: Vec<ResourceId> = res_id_iter.collect();

        // Transform iter: from (ResourceId) to (FileType, ResourceId)
        let ft_and_rid_iter = trafo_to_filetype_and_resource_id(
            all_resource_ids.clone().into_iter(),
            &meta_data_loader,
        );

        // Filter markdown files
        let md_resource_id_iter = filter_markdown_types(ft_and_rid_iter);
        let md_resource_ids: Vec<ResourceId> = md_resource_id_iter.collect();

        debug!("Creation of Indexes took: {:?}", start.elapsed());

        let start = Instant::now();
        let resource_id_retriever = ResourceIdLinkMap::new(all_resource_ids.iter());
        debug!("Creation of ResourceIdLinkMap took: {:?}", start.elapsed());

        let start = Instant::now();
        let content_cache = ContentFullMdCache::new(md_resource_ids.iter(), content_loader.clone());
        debug!("Creation of ContentFullMdCache took: {:?}", start.elapsed());

        let start = Instant::now();
        let all_links_iter = extract_links_from_vault(
            md_resource_ids.iter(),
            &content_cache,
            &resource_id_retriever,
            &analyze_markdown,
        );
        let src_2_tgt_iter_src = Src2TargetIndex::new(all_links_iter);

        debug!("Creation of Src2TargetIndex took: {:?}", start.elapsed());

        let start = Instant::now();
        let tgt_iter_retriever = TgtLinksMap::new(src_2_tgt_iter_src.iter());
        debug!("Creation of TgtLinksMap took: {:?}", start.elapsed());

        let start = Instant::now();
        let src_iter_retriever = SrcLinksMap::new(src_2_tgt_iter_src.iter());
        debug!("Creation of SrcLinksMap took: {:?}", start.elapsed());

        let start = Instant::now();
        let provider_factory =
            StdProviderFactory::new(meta_data_loader.clone(), content_cache.clone());
        debug!("Creation of StdProviderFactory took: {:?}", start.elapsed());

        let start = Instant::now();
        let vault = Vault::new(md_resource_ids.iter(), provider_factory.clone());
        debug!("Creation of Vault took: {:?}", start.elapsed());

        Ok(Emerald {
            resource_id_resolver,
            endpoint_resolver,
            meta_data_loader,
            resource_id_retriever,
            ep_iter_src,
            content_cache,
            src_2_tgt_iter_src,
            tgt_iter_retriever,
            src_iter_retriever,
            provider_factory,
            vault,
        })
    }
}

impl Emerald {
    pub fn get_vault(&self) -> Vault<StdProviderFactoryImpl> {
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
        self.src_2_tgt_iter_src.get_valid_backlink_cnt()
    }

    pub fn invalid_backlink_count(&self) -> usize {
        self.src_2_tgt_iter_src.get_invalid_backlink_cnt()
    }
}
