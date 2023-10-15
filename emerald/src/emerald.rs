use crate::indexes::Src2TargetIndex;
use crate::maps::resource_id_link_map::ResourceIdLinkMap;
use crate::maps::src_links_map::SrcLinksMap;
use crate::maps::tgt_links_map::TgtLinksMap;
use crate::md_analyzer::analyze_markdown;
use crate::notes::providers::std_provider_factory::StdProviderFactory;
use crate::notes::vault::Vault;
use crate::resources;
use crate::resources::endpoint_resource_id_map::EndpointResourceIdMap;
use crate::resources::file_content_loader::FileContentLoader;
use crate::resources::md_content_cache::MdContentCache;
use crate::resources::resource_id_endpoint_map::ResourceIdEndPointMap;
use crate::trafos;
use crate::types::{EndPoint, ResourceId};
use crate::Result;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{path::Path, time::Instant};

type FileMetaDataLoaderImpl = resources::FileMetaDataLoader<EndpointResourceIdMap>;
type StdProviderFactoryImpl = StdProviderFactory<FileMetaDataLoaderImpl, MdContentCache>;

#[allow(dead_code)]
pub struct Emerald {
    pub ep_index: Vec<EndPoint>,
    pub resource_id_resolver: ResourceIdEndPointMap,
    pub endpoint_resolver: EndpointResourceIdMap,
    pub meta_data_loader: FileMetaDataLoaderImpl,
    pub resource_id_retriever: ResourceIdLinkMap,
    pub src_2_tgt_index: Src2TargetIndex,
    pub md_content_cache: MdContentCache,
    pub tgt_iter_retriever: TgtLinksMap,
    pub src_iter_retriever: SrcLinksMap,
    pub provider_factory: StdProviderFactoryImpl,
    pub vault: Vault<StdProviderFactoryImpl>,
}

impl Emerald {
    pub fn new(vault_path: &Path) -> Result<Emerald> {
        // Build dependency root
        let start = Instant::now();
        let path_list = resources::get_path_list(vault_path)?;
        let ep_index: Vec<_> = resources::trafo_from_pathes_to_endpoints(path_list).collect();
        debug!("Creation of EndpointIndex took: {:?}", start.elapsed());

        let start = Instant::now();
        let resource_id_resolver = ResourceIdEndPointMap::new(ep_index.iter(), vault_path);
        debug!(
            "Creation of ResourceIdEndPointMap took: {:?}",
            start.elapsed()
        );

        let start = Instant::now();
        let endpoint_resolver = EndpointResourceIdMap::new(ep_index.iter(), &resource_id_resolver);
        debug!(
            "Creation of EndpointResourceIdMap took: {:?}",
            start.elapsed()
        );

        let start = Instant::now();
        let content_loader = FileContentLoader::new(endpoint_resolver.clone());
        debug!("Creation of FileContentLoader took: {:?}", start.elapsed());

        let start = Instant::now();
        let meta_data_loader = resources::FileMetaDataLoader::new(endpoint_resolver.clone());
        debug!("Creation of FileMetaDataLoader took: {:?}", start.elapsed());

        let start = Instant::now();
        let res_id_iter = trafos::trafo_ep_to_rid(ep_index.iter(), &resource_id_resolver);
        let all_res_ids: Vec<ResourceId> = res_id_iter.collect();

        // Transform iter: from (ResourceId) to (FileType, ResourceId)
        let ft_and_rid_iter = trafos::trafo_to_filetype_and_resource_id(
            all_res_ids.clone().into_iter(),
            &meta_data_loader,
        );

        // Filter markdown files
        let md_res_ids_iter = trafos::filter_markdown_types(ft_and_rid_iter);
        let md_res_ids: Vec<ResourceId> = md_res_ids_iter.collect();

        debug!(
            "Creation of Resource Id indexes took: {:?}",
            start.elapsed()
        );

        let start = Instant::now();
        let name_iter = trafos::trafo_from_res_id_to_name(all_res_ids.iter());
        let resource_id_retriever = ResourceIdLinkMap::new(name_iter);
        debug!("Creation of ResourceIdLinkMap took: {:?}", start.elapsed());

        let start = Instant::now();
        let md_content_cache = MdContentCache::new(md_res_ids.iter(), &content_loader);
        debug!("Creation of ContentFullMdCache took: {:?}", start.elapsed());

        let start = Instant::now();
        let content_iter =
            trafos::trafo_from_res_ids_to_content(md_res_ids.iter(), &md_content_cache);
        let src_2_tgt_iter = trafos::trafo_from_content_to_linksrc2tgt(
            content_iter,
            &resource_id_retriever,
            &analyze_markdown,
        );
        let src_2_tgt_index = Src2TargetIndex::new(src_2_tgt_iter);

        debug!("Creation of Src2TargetIndex took: {:?}", start.elapsed());

        let start = Instant::now();
        let tgt_iter_retriever = TgtLinksMap::new(src_2_tgt_index.iter());
        debug!("Creation of TgtLinksMap took: {:?}", start.elapsed());

        let start = Instant::now();
        let src_iter_retriever = SrcLinksMap::new(src_2_tgt_index.iter());
        debug!("Creation of SrcLinksMap took: {:?}", start.elapsed());

        let start = Instant::now();
        let provider_factory =
            StdProviderFactory::new(meta_data_loader.clone(), md_content_cache.clone());
        debug!("Creation of StdProviderFactory took: {:?}", start.elapsed());

        let start = Instant::now();
        let vault = Vault::new(md_res_ids.iter(), provider_factory.clone());
        debug!("Creation of Vault took: {:?}", start.elapsed());

        Ok(Emerald {
            resource_id_resolver,
            endpoint_resolver,
            meta_data_loader,
            resource_id_retriever,
            ep_index,
            md_content_cache,
            src_2_tgt_index,
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
        self.ep_index.iter().count()
    }

    pub fn md_file_count(&self) -> usize {
        self.ep_index
            .iter()
            .filter(|pred| matches!(pred, EndPoint::FileMarkdown(_)))
            .count()
    }

    pub fn valid_backlink_count(&self) -> usize {
        self.src_2_tgt_index.get_valid_backlink_cnt()
    }

    pub fn invalid_backlink_count(&self) -> usize {
        self.src_2_tgt_index.get_invalid_backlink_cnt()
    }
}
