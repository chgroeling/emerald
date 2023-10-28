use super::adapters;
use super::error::Result;
use super::markdown;
use super::model;
use super::notes;
use super::resources;
use super::stats;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::rc::Rc;
use std::{path::Path, time::Instant};

type FileMetaDataLoaderImpl = resources::FileMetaDataLoader<resources::ResourceObjectMap>;
type StdProviderFactoryImpl = notes::StdProviderFactory<resources::MdContentCache>;

#[allow(dead_code)]
pub struct Emerald {
    pub rid_retriever: resources::ResourceIdMap,
    pub ro_retriever: resources::ResourceObjectMap,
    pub meta_data_loader: FileMetaDataLoaderImpl,
    pub md_content_cache: resources::MdContentCache,
    pub provider_factory: StdProviderFactoryImpl,
    pub vault: notes::Vault<
        StdProviderFactoryImpl,
        <model::DefaultNoteModel as model::NotesIterSrc>::Iter,
    >,
    pub vault_stats: stats::VaultStats,
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
        let md_content_cache = resources::MdContentCache::new(&md_index, &content_loader);
        let elapsed = start.elapsed();
        debug!("Creation of ContentFullMdCache took: {:?}", elapsed);

        let start = Instant::now();
        let name_iter = adapters::adapter_from_rid_to_name(&all_index)?;
        let link_model = Rc::new(model::DefaultLinkModel::new(name_iter));
        let elapsed = start.elapsed();
        debug!("Creation of DefaultLinkModel took: {:?}", elapsed);

        let start = Instant::now();
        let c_it = adapters::adapter_from_rids_to_rids_and_content(&md_index, &md_content_cache)?;
        let md_analyzer = markdown::MarkdownAnalyzerImpl::new();
        let ct_it = adapters::adapter_to_rid_and_content_type(c_it, md_analyzer);
        let src2tgt_idx: Vec<_> =
            adapters::adapter_to_link_src_2_tgt(ct_it, link_model.as_ref()).collect();
        let elapsed = start.elapsed();
        debug!("Creation of sources to target index took: {:?}", elapsed);

        let start = Instant::now();
        let fmod = Rc::new(model::DefaultFileModel::new(all_index));
        let elapsed = start.elapsed();
        debug!("Creation of DefaultFileModel took: {:?}", elapsed);

        let start = Instant::now();
        // load all meta data and ensure that there were no errors
        let md_meta_data = adapters::adapter_to_rid_and_meta_data(md_index, &meta_data_loader)?;
        let nmod = Rc::new(model::DefaultNoteModel::new(md_meta_data, src2tgt_idx));
        let elapsed = start.elapsed();
        debug!("Creation of DefaultNoteModel took: {:?}", elapsed);

        let start = Instant::now();
        let provider_factory =
            notes::StdProviderFactory::new(nmod.clone(), md_content_cache.clone());
        let elapsed = start.elapsed();
        debug!("Creation of StdProviderFactory took: {:?}", elapsed);

        let start = Instant::now();
        let vault = notes::Vault::new(nmod.clone(), provider_factory.clone());
        let elapsed = start.elapsed();
        debug!("Creation of Vault took: {:?}", elapsed);

        // -----
        // Aquire stats
        let link_stats = stats::extract_link_stats(nmod.as_ref());
        let file_stats = stats::extract_file_stats(fmod.as_ref(), nmod.as_ref());
        let vault_stats = stats::VaultStats {
            file_stats,
            link_stats,
        };
        // -------
        Ok(Emerald {
            rid_retriever,
            ro_retriever,
            meta_data_loader,
            md_content_cache,
            provider_factory,
            vault,
            vault_stats,
        })
    }
}

impl Emerald {
    pub fn get_vault(
        &self,
    ) -> notes::Vault<StdProviderFactoryImpl, <model::DefaultNoteModel as model::NotesIterSrc>::Iter>
    {
        self.vault.clone()
    }

    pub fn file_count(&self) -> usize {
        self.vault_stats.file_stats.file_count
    }

    pub fn md_file_count(&self) -> usize {
        self.vault_stats.file_stats.md_file_count
    }

    pub fn valid_backlink_count(&self) -> usize {
        self.vault_stats.link_stats.valid_backlinks
    }

    pub fn invalid_backlink_count(&self) -> usize {
        self.vault_stats.link_stats.invalid_backlinks
    }
}
