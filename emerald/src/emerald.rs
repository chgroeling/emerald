use crate::resources::FsMetaDataAccessImpl;

use super::adapters;
use super::error::Result;
use super::markdown;
use super::model::content;
use super::model::link;
use super::model::note;
use super::model::resource;
use super::model::resource_id_resolver;
use super::resources;
use super::stats;
use super::vault;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::rc::Rc;
use std::{path::Path, time::Instant};

#[allow(dead_code)]
pub struct Emerald {
    pub vault: vault::VaultImpl<<note::DefaultNoteModel as note::NotesIterSrc>::Iter>,
    pub stats: stats::VaultStats,
}

impl Emerald {
    pub fn new(vault_path: &Path) -> Result<Emerald> {
        // Build dependency root
        let start = Instant::now();
        let path_list = resources::get_path_list(vault_path)?;
        let all_ros_vec: Vec<_> = resources::adapter_to_ro(path_list).collect();
        let ros_rids: Vec<_> = resources::adapter_to_ro_and_rid(all_ros_vec, vault_path)?.collect();
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
        let all_vec: Vec<_> = resources::adapter_to_rid(ros_rids).collect();
        let elapsed = start.elapsed();
        debug!("Creation of ResourceId vec: {:?}", elapsed);

        let start = Instant::now();
        let content_loader = resources::FileContentLoader::new(ro_retriever.clone());
        let elapsed = start.elapsed();
        debug!("Creation of FileContentLoader: {:?}", elapsed);

        let start = Instant::now();
        let meta_data_loader =
            resources::FileMetaDataLoader::new(ro_retriever.clone(), FsMetaDataAccessImpl());
        let elapsed = start.elapsed();
        debug!("Creation of FileMetaDataLoader: {:?}", elapsed);

        let start = Instant::now();

        // load all meta data and ensure that there were no errors
        let all_meta_data: Vec<_> =
            adapters::adapter_to_rid_and_meta_data(&all_vec, &meta_data_loader)?.collect();

        let md_meta_data: Vec<_> = adapters::filter_rid_and_meta_data(&all_meta_data).collect();
        let md_notes: Vec<_> = md_meta_data.iter().map(|f| f.0.clone()).collect();

        let elapsed = start.elapsed();
        debug!("Creation of ResourceId md vec: {:?}", elapsed);

        let start = Instant::now();
        let md_content_vec =
            resources::adapter_to_rid_and_content(md_notes.iter(), &content_loader)?;
        let cmod = Rc::new(content::DefaultContentModel::new(md_content_vec));
        let elapsed = start.elapsed();
        debug!("Creation of DefaultContentModel: {:?}", elapsed);

        let start = Instant::now();
        let name_iter = adapters::adapter_to_rid_and_name(&all_vec)?;
        let lrmod = Rc::new(resource_id_resolver::DefaultResourceIdResolverModel::new(
            name_iter,
        ));
        let elapsed = start.elapsed();
        debug!("Creation of DefaultResourceIdResolverModel: {:?}", elapsed);

        let start = Instant::now();
        let rmod = Rc::new(resource::DefaultResourceModel::new(all_meta_data));
        let elapsed = start.elapsed();
        debug!("Creation of DefaultResourceModel: {:?}", elapsed);

        let start = Instant::now();
        let md_analyzer = markdown::MarkdownAnalyzerImpl::new();
        let c_it = adapters::adapter_to_rids_and_content(md_notes.iter(), cmod.as_ref());
        let ct_it = adapters::adapter_to_rid_and_content_type(c_it, md_analyzer);
        let s2t_idx: Vec<_> = adapters::adapter_to_link_src_2_tgt(ct_it, lrmod.as_ref()).collect();
        let elapsed = start.elapsed();
        debug!("Link and Backlink extraction: {:?}", elapsed);

        let start = Instant::now();
        let md_analyzer = markdown::MarkdownAnalyzerImpl::new();
        let c_it = adapters::adapter_to_rids_and_content(md_notes.iter(), cmod.as_ref());
        let ct_it = adapters::adapter_to_yaml(c_it, md_analyzer);
        /*
        for i in ct_it {
            let yaml = i.1;
            println!("{yaml}");
        }
        */
        let yaml_meta_data: Vec<_> = adapters::adapter_to_btree(ct_it).collect();
        let elapsed = start.elapsed();
        debug!("YAML extraction: {:?}", elapsed);

        let start = Instant::now();
        let lmod = Rc::new(link::DefaultLinkModel::new(s2t_idx));
        let elapsed = start.elapsed();
        debug!("Creation of DefaultLinkModel: {:?}", elapsed);

        let start = Instant::now();
        let nmod = Rc::new(note::DefaultNoteModel::new(md_meta_data));
        let elapsed = start.elapsed();
        debug!("Creation of DefaultNoteModel: {:?}", elapsed);

        let start = Instant::now();
        let note_factory = Rc::new(vault::NoteFactoryImpl::new(nmod.clone(), cmod.clone()));
        let elapsed = start.elapsed();
        debug!("Creation of NoteFactoryImpl: {:?}", elapsed);

        let start = Instant::now();
        let vault = vault::VaultImpl::new(
            note_factory,
            nmod.clone(),
            lmod.clone(),
            lmod.clone(),
            rmod.clone(),
        );
        let elapsed = start.elapsed();
        debug!("Creation of Vault: {:?}", elapsed);

        // -----
        // Aquire stats
        let link_stats = stats::extract_link_stats(lmod.as_ref());
        let file_stats = stats::extract_file_stats(rmod.as_ref(), nmod.as_ref());
        let vault_stats = stats::VaultStats {
            file_stats,
            link_stats,
        };
        // -------
        Ok(Emerald {
            vault,
            stats: vault_stats,
        })
    }
}

impl Emerald {
    pub fn get_vault(
        &self,
    ) -> vault::VaultImpl<<note::DefaultNoteModel as note::NotesIterSrc>::Iter> {
        self.vault.clone()
    }

    pub fn file_count(&self) -> usize {
        self.stats.file_stats.file_count
    }

    pub fn md_file_count(&self) -> usize {
        self.stats.file_stats.md_file_count
    }

    pub fn valid_backlink_count(&self) -> usize {
        self.stats.link_stats.valid_backlinks
    }

    pub fn invalid_backlink_count(&self) -> usize {
        self.stats.link_stats.invalid_backlinks
    }
}
