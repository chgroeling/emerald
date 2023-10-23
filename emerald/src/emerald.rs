use super::adapters;
use super::error::Result;
use super::indexes;
use super::maps;
use super::markdown;
use super::notes;
use super::resources;
use super::types;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{path::Path, time::Instant};

type FileMetaDataLoaderImpl = resources::FileMetaDataLoader<resources::ResourceObjectMap>;
type StdProviderFactoryImpl =
    notes::StdProviderFactory<FileMetaDataLoaderImpl, resources::MdContentCache>;

#[allow(dead_code)]
pub struct Emerald {
    pub all_index: Vec<types::ResourceId>,
    pub md_index: Vec<types::ResourceId>,
    pub rid_retriever: resources::ResourceIdMap,
    pub ro_retriever: resources::ResourceObjectMap,
    pub meta_data_loader: FileMetaDataLoaderImpl,
    pub rid_resolver: maps::ResourceIdLinkMap,
    pub src_2_tgt_index: indexes::Src2TargetIndex,
    pub md_content_cache: resources::MdContentCache,
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
        let all_ros: Vec<_> = resources::adapter_from_pathes_to_ro(path_list).collect();
        debug!("Creation of EndpointIndex took: {:?}", start.elapsed());

        let start = Instant::now();
        let ros_and_rids: Vec<_> =
            resources::adapter_ro_to_ro_and_rid(&all_ros, vault_path)?.collect();

        let res_id_iter = resources::adapter_ro_to_rid(&ros_and_rids);
        let all_index: Vec<_> = res_id_iter.collect();

        let elapsed = start.elapsed();
        debug!(
            "Creation of ResourceObject and ResourceId lists took: {:?}",
            elapsed
        );

        let start = Instant::now();
        let rid_retriever = resources::ResourceIdMap::new(&ros_and_rids)?;
        let elapsed = start.elapsed();
        debug!("Creation of ResourceIdEndPointMap took: {:?}", elapsed);

        let start = Instant::now();
        let ro_retriever = resources::ResourceObjectMap::new(&ros_and_rids)?;
        let elapsed = start.elapsed();
        debug!("Creation of EndpointResourceIdMap took: {:?}", elapsed);

        let start = Instant::now();
        let content_loader = resources::FileContentLoader::new(ro_retriever.clone());
        let elapsed = start.elapsed();
        debug!("Creation of FileContentLoader took: {:?}", elapsed);

        let start = Instant::now();
        let meta_data_loader = resources::FileMetaDataLoader::new(ro_retriever.clone());
        let elapsed = start.elapsed();
        debug!("Creation of FileMetaDataLoader took: {:?}", elapsed);

        let start = Instant::now();
        // Transform iter: from (ResourceId) to (FileType, ResourceId)
        let ft_and_rid_iter = adapters::adapter_to_rid_and_filetype(&all_index, &meta_data_loader);

        // Filter markdown files
        let md_rids_iter = adapters::adapter_rid_and_file_type_to_rid(ft_and_rid_iter);
        let md_index: Vec<_> = md_rids_iter.cloned().collect();
        let elapsed = start.elapsed();
        debug!("Creation of Resource Id indexes took: {:?}", elapsed);

        let start = Instant::now();
        let name_iter = adapters::adapter_from_rid_to_name(&all_index)?;
        let rid_resolver = maps::ResourceIdLinkMap::new(name_iter);
        let elapsed = start.elapsed();
        debug!("Creation of ResourceIdLinkMap took: {:?}", elapsed);

        let start = Instant::now();
        let md_content_cache = resources::MdContentCache::new(&md_index, &content_loader);
        let elapsed = start.elapsed();
        debug!("Creation of ContentFullMdCache took: {:?}", elapsed);

        let start = Instant::now();
        let crefs: Vec<_> =
            adapters::adapter_from_rids_to_rids_and_content(&md_index, &md_content_cache)?
                .collect();

        let src_2_tgt_iter = adapters::adapter_from_rid_and_content_to_link_src_2_tgt(
            crefs,
            &rid_resolver,
            markdown::MarkdownAnalyzerLocal::new(),
        );

        let src_2_tgt_index = indexes::Src2TargetIndex::new(src_2_tgt_iter);
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
        let vault = notes::Vault::new(&md_index, provider_factory.clone());
        let elapsed = start.elapsed();
        debug!("Creation of Vault took: {:?}", elapsed);

        Ok(Emerald {
            rid_retriever,
            ro_retriever,
            meta_data_loader,
            rid_resolver,
            md_index,
            all_index,
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
        self.all_index.len()
    }

    pub fn md_file_count(&self) -> usize {
        self.md_index.iter().count()
    }

    pub fn valid_backlink_count(&self) -> usize {
        self.src_2_tgt_index.get_valid_backlink_cnt()
    }

    pub fn invalid_backlink_count(&self) -> usize {
        self.src_2_tgt_index.get_invalid_backlink_cnt()
    }
}
