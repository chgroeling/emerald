use super::adapters;
use super::error::Result;
use super::markdown;
use super::model::content;
use super::model::file;
use super::model::link;
use super::model::note;
use super::notes;
use super::resources;
use super::stats;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::rc::Rc;
use std::{path::Path, time::Instant};

type StdProviderFactoryImpl = notes::StdProviderFactory;

#[allow(dead_code)]
pub struct Emerald {
    pub vault:
        notes::Vault<StdProviderFactoryImpl, <note::DefaultNoteModel as note::NotesIterSrc>::Iter>,
    pub vault_stats: stats::VaultStats,
}

impl Emerald {
    pub fn new(vault_path: &Path) -> Result<Emerald> {
        // Build dependency root
        let start = Instant::now();
        let path_list = resources::get_path_list(vault_path)?;
        let all_ros_vec: Vec<_> = resources::adapter_from_pathes_to_ro(path_list).collect();
        debug!("Creation of ResourceObject vec: {:?}", start.elapsed());

        let start = Instant::now();
        let ros_rids: Vec<_> =
            resources::adapter_ro_to_ro_and_rid(all_ros_vec, vault_path)?.collect();

        let all_vec: Vec<_> = resources::adapter_to_rid(&ros_rids).collect();
        let elapsed = start.elapsed();
        debug!("Creation of ResourceId vec: {:?}", elapsed);

        let start = Instant::now();
        let _rid_retriever = resources::ResourceIdMap::new(&ros_rids);
        let elapsed = start.elapsed();
        debug!("Creation of ResourceIdMap: {:?}", elapsed);

        let start = Instant::now();
        let ro_retriever = resources::ResourceObjectMap::new(&ros_rids);
        let elapsed = start.elapsed();
        debug!("Creation of ResourceObjectMap: {:?}", elapsed);

        let start = Instant::now();
        let content_loader = resources::FileContentLoader::new(ro_retriever.clone());
        let elapsed = start.elapsed();
        debug!("Creation of FileContentLoader: {:?}", elapsed);

        let start = Instant::now();
        let meta_data_loader = resources::FileMetaDataLoader::new(ro_retriever.clone());
        let elapsed = start.elapsed();
        debug!("Creation of FileMetaDataLoader: {:?}", elapsed);

        let start = Instant::now();

        // load all meta data and ensure that there were no errors
        let all_meta_data: Vec<_> =
            adapters::adapter_to_rid_and_meta_data(all_vec.clone(), &meta_data_loader)?.collect();

        // Transform iter: from (ResourceId) to (FileType, ResourceId)
        let md_vec_md: Vec<_> = adapters::adapter_to_rid(all_meta_data).collect();
        let md_vec: Vec<_> = md_vec_md.iter().map(|f| f.0.clone()).collect();

        let elapsed = start.elapsed();
        debug!("Creation of ResourceId md vec: {:?}", elapsed);

        let start = Instant::now();
        let md_content_vec = resources::adapter_to_rid_and_content(&md_vec, &content_loader)?;
        let content_model = Rc::new(content::DefaultContentModel::new(md_content_vec));
        let elapsed = start.elapsed();
        debug!("Creation of DefaultContentModel: {:?}", elapsed);

        let start = Instant::now();
        let name_iter = adapters::adapter_from_rid_to_name(&all_vec)?;
        let link_model = Rc::new(link::DefaultLinkModel::new(name_iter));
        let elapsed = start.elapsed();
        debug!("Creation of DefaultLinkModel: {:?}", elapsed);

        let start = Instant::now();

        let fmod = Rc::new(file::DefaultFileModel::new(all_vec));
        let elapsed = start.elapsed();
        debug!("Creation of DefaultFileModel: {:?}", elapsed);

        let start = Instant::now();
        let c_it = adapters::adapter_to_rids_and_content(&md_vec, content_model.as_ref());
        let md_analyzer = markdown::MarkdownAnalyzerImpl::new();
        let ct_it = adapters::adapter_to_rid_and_content_type(c_it, md_analyzer);
        let s2t_idx: Vec<_> =
            adapters::adapter_to_link_src_2_tgt(ct_it, link_model.as_ref()).collect();

        let nmod = Rc::new(note::DefaultNoteModel::new(md_vec_md, s2t_idx));
        let elapsed = start.elapsed();
        debug!("Creation of DefaultNoteModel: {:?}", elapsed);

        let start = Instant::now();
        let provider_factory = notes::StdProviderFactory::new(nmod.clone(), content_model.clone());
        let elapsed = start.elapsed();
        debug!("Creation of StdProviderFactory: {:?}", elapsed);

        let start = Instant::now();
        let vault = notes::Vault::new(nmod.clone(), provider_factory.clone());
        let elapsed = start.elapsed();
        debug!("Creation of Vault: {:?}", elapsed);

        // -----
        // Aquire stats
        let link_stats = stats::extract_link_stats(nmod.as_ref());
        let file_stats = stats::extract_file_stats(fmod.as_ref(), nmod.as_ref());
        let vault_stats = stats::VaultStats {
            file_stats,
            link_stats,
        };
        // -------
        Ok(Emerald { vault, vault_stats })
    }
}

impl Emerald {
    pub fn get_vault(
        &self,
    ) -> notes::Vault<StdProviderFactoryImpl, <note::DefaultNoteModel as note::NotesIterSrc>::Iter>
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
