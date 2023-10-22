use crate::adapters;
use crate::indexes::Src2TargetIndex;
use crate::maps;
use crate::markdown::MarkdownAnalyzerLocal;
use crate::notes;
use crate::resources;
use crate::resources::endpoint_resource_id_map::EndpointResourceIdMap;
use crate::resources::file_content_loader::FileContentLoader;
use crate::resources::md_content_cache::MdContentCache;
use crate::resources::resource_id_endpoint_map::ResourceIdEndPointMap;
use crate::types::EndPoint;
use crate::Result;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{path::Path, time::Instant};

type FileMetaDataLoaderImpl = resources::FileMetaDataLoader<EndpointResourceIdMap>;
type StdProviderFactoryImpl = notes::StdProviderFactory<FileMetaDataLoaderImpl, MdContentCache>;

#[allow(dead_code)]
pub struct Emerald {
    pub ep_index: Vec<EndPoint>,
    pub rid_retriever: ResourceIdEndPointMap,
    pub ep_retriever: EndpointResourceIdMap,
    pub meta_data_loader: FileMetaDataLoaderImpl,
    pub rid_resolver: maps::ResourceIdLinkMap,
    pub src_2_tgt_index: Src2TargetIndex,
    pub md_content_cache: MdContentCache,
    pub tgt_iter_retriever: maps::TgtLinksMap,
    pub src_iter_retriever: maps::SrcLinksMap,
    pub provider_factory: StdProviderFactoryImpl,
    pub vault: notes::Vault<StdProviderFactoryImpl>,
}

impl Emerald {
    pub fn new(vault_path: &Path) -> Result<Emerald> {
        // Build dependency root
        let start = Instant::now();
        let path_list = resources::get_path_list(vault_path)?;
        let all_eps: Vec<_> = resources::adapter_from_pathes_to_ep(path_list).collect();
        debug!("Creation of EndpointIndex took: {:?}", start.elapsed());

        let start = Instant::now();
        let ep_and_rids: Vec<_> =
            resources::adapter_ep_to_ep_and_rid(&all_eps, vault_path)?.collect();

        let res_id_iter = resources::adapter_ep_to_rid(&ep_and_rids);
        let all_rids: Vec<_> = res_id_iter.collect();

        let elapsed = start.elapsed();
        debug!(
            "Creation of Endpoint with Resource Id lists took: {:?}",
            elapsed
        );

        let start = Instant::now();
        let rid_retriever = ResourceIdEndPointMap::new(&ep_and_rids)?;
        let elapsed = start.elapsed();
        debug!("Creation of ResourceIdEndPointMap took: {:?}", elapsed);

        let start = Instant::now();
        let ep_retriever = EndpointResourceIdMap::new(&ep_and_rids)?;
        let elapsed = start.elapsed();
        debug!("Creation of EndpointResourceIdMap took: {:?}", elapsed);

        let start = Instant::now();
        let content_loader = FileContentLoader::new(ep_retriever.clone());
        let elapsed = start.elapsed();
        debug!("Creation of FileContentLoader took: {:?}", elapsed);

        let start = Instant::now();
        let meta_data_loader = resources::FileMetaDataLoader::new(ep_retriever.clone());
        let elapsed = start.elapsed();
        debug!("Creation of FileMetaDataLoader took: {:?}", elapsed);

        let start = Instant::now();
        // Transform iter: from (ResourceId) to (FileType, ResourceId)
        let ft_and_rid_iter = adapters::adapter_to_rid_and_filetype(&all_rids, &meta_data_loader);

        // Filter markdown files
        let md_rids_iter = adapters::adapter_rid_and_file_type_to_rid(ft_and_rid_iter);
        let md_rids: Vec<_> = md_rids_iter.cloned().collect();
        let elapsed = start.elapsed();
        debug!("Creation of Resource Id indexes took: {:?}", elapsed);

        let start = Instant::now();
        let name_iter = adapters::adapter_from_rid_to_name(&all_rids)?;
        let rid_resolver = maps::ResourceIdLinkMap::new(name_iter);
        let elapsed = start.elapsed();
        debug!("Creation of ResourceIdLinkMap took: {:?}", elapsed);

        let start = Instant::now();
        let md_content_cache = MdContentCache::new(&md_rids, &content_loader);
        let elapsed = start.elapsed();
        debug!("Creation of ContentFullMdCache took: {:?}", elapsed);

        let start = Instant::now();
        let crefs: Vec<_> =
            adapters::adapter_from_rids_to_rids_and_content(&md_rids, &md_content_cache)?.collect();

        let src_2_tgt_iter = adapters::adapter_from_rid_and_content_to_link_src_2_tgt(
            crefs.into_iter(),
            &rid_resolver,
            MarkdownAnalyzerLocal::new(),
        );

        let src_2_tgt_index = Src2TargetIndex::new(src_2_tgt_iter);
        let elapsed = start.elapsed();
        debug!("Creation of Src2TargetIndex took: {:?}", elapsed);

        let start = Instant::now();
        let tgt_iter_retriever = maps::TgtLinksMap::new(&src_2_tgt_index);
        let elapsed = start.elapsed();
        debug!("Creation of TgtLinksMap took: {:?}", elapsed);

        let start = Instant::now();
        let src_iter_retriever = maps::SrcLinksMap::new(&src_2_tgt_index);
        let elapsed = start.elapsed();
        debug!("Creation of SrcLinksMap took: {:?}", elapsed);

        let start = Instant::now();
        let provider_factory =
            notes::StdProviderFactory::new(meta_data_loader.clone(), md_content_cache.clone());
        let elapsed = start.elapsed();
        debug!("Creation of StdProviderFactory took: {:?}", elapsed);

        let start = Instant::now();
        let vault = notes::Vault::new(&md_rids, provider_factory.clone());
        let elapsed = start.elapsed();
        debug!("Creation of Vault took: {:?}", elapsed);

        Ok(Emerald {
            rid_retriever,
            ep_retriever,
            meta_data_loader,
            rid_resolver,
            ep_index: all_eps,
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
    pub fn get_vault(&self) -> notes::Vault<StdProviderFactoryImpl> {
        self.vault.clone()
    }

    pub fn file_count(&self) -> usize {
        self.ep_index.len()
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
