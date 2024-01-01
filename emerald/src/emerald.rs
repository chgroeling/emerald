use crate::model::note::NotesIterSrc;
use crate::resources::FsMetadataAccessImpl;

use super::adapters;
use super::error::Result;
use super::markdown;
use super::model::content;
use super::model::link;
use super::model::note;
use super::model::note_writer;
use super::model::resource;
use super::model::resource_id_resolver;
use super::model::vault;
use super::resources;
use super::stats;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::iter::zip;
use std::rc::Rc;
use std::{path::Path, time::Instant};

#[allow(dead_code)]
pub struct Emerald {
    pub vault: vault::VaultImpl,
    pub stats: stats::VaultStats,
}

impl Emerald {
    pub fn new(vault_path: &Path) -> Result<Emerald> {
        // Build dependency root
        let start = Instant::now();
        let mut path_list = resources::get_path_list(vault_path)?;
        path_list.sort_by(|a, b| a.file_stem().cmp(&b.file_stem()));
        let all_ros_vec: Vec<_> = resources::adapter_to_ro(path_list).collect();
        let elapsed = start.elapsed();
        debug!("Creation of ResourceObject vec: {:?}", elapsed);

        let start = Instant::now();
        let ros_rids: Vec<_> = resources::adapter_to_ro_and_rid(&all_ros_vec, vault_path).collect();
        let elapsed = start.elapsed();
        debug!(
            "Creation of (ResourceObject, ResourceId) vec: {:?}",
            elapsed
        );

        let start = Instant::now();
        let ro_retriever = resources::ResourceObjectMap::new(&ros_rids);
        let elapsed = start.elapsed();
        debug!("Creation of ResourceObjectMap: {:?}", elapsed);

        let start = Instant::now();
        let all_vec: Vec<_> = ros_rids.into_iter().map(|(_, rid)| rid).collect();
        let elapsed = start.elapsed();
        debug!("Creation of ResourceId vec: {:?}", elapsed);

        let start = Instant::now();
        let content_loader = resources::FileContentLoader::new(ro_retriever.clone());
        let elapsed = start.elapsed();
        debug!("Creation of FileContentLoader: {:?}", elapsed);

        let start = Instant::now();
        let fs_meta_data_loader = resources::FilesystemMetadataLoaderImpl::new(
            ro_retriever.clone(),
            FsMetadataAccessImpl(),
        );
        let elapsed = start.elapsed();
        debug!("Creation of FilesystemMetadataLoader: {:?}", elapsed);

        let start = Instant::now();

        // load all meta data and ensure that there were no errors
        let all_fs_meta_data: Vec<_> = adapters::adapter_to_rid_and_filesystem_metadata(
            all_vec.clone(),
            &fs_meta_data_loader,
        )?
        .collect();

        let md_fs_meta_data: Vec<_> =
            adapters::filter_rid_and_meta_data(&all_fs_meta_data).collect();

        let md_rids: Vec<_> = md_fs_meta_data.iter().map(|f| f.0.clone()).collect();
        let elapsed = start.elapsed();
        debug!("Creation of ResourceId md vec: {:?}", elapsed);

        let start = Instant::now();
        let md_content_vec =
            resources::adapter_to_rid_and_content(md_rids.iter(), &content_loader)?;
        let cmod = Rc::new(content::DefaultContentModel::new(md_content_vec));
        let elapsed = start.elapsed();
        debug!("Creation of DefaultContentModel: {:?}", elapsed);

        let start = Instant::now();
        let md_analyzer = markdown::MarkdownAnalyzerImpl::new();
        let c_it = adapters::adapter_to_rids_and_content(md_rids.iter(), cmod.as_ref());
        let ct_it = adapters::adapter_to_rid_and_yaml(c_it, md_analyzer);
        let md_doc_meta_data: Vec<_> =
            adapters::adapter_to_rid_and_document_metadata(ct_it).collect();
        let elapsed = start.elapsed();
        debug!("YAML extraction: {:?}", elapsed);

        let start = Instant::now();
        let resource_loc_iter = adapters::to_resource_id_resolver::convert_to_resource_locations(
            &all_fs_meta_data,
            vault_path,
        );
        let lrmod = Rc::new(resource_id_resolver::DefaultResourceIdResolverModel::new(
            resource_loc_iter,
        ));
        let elapsed = start.elapsed();
        debug!("Creation of DefaultResourceIdResolverModel: {:?}", elapsed);

        let start = Instant::now();
        let rmod = Rc::new(resource::DefaultResourceModel::new(&all_fs_meta_data));
        let elapsed = start.elapsed();
        debug!("Creation of DefaultResourceModel: {:?}", elapsed);

        let start = Instant::now();
        let md_analyzer = markdown::MarkdownAnalyzerImpl::new();
        let c_it = adapters::adapter_to_rids_and_content(md_rids.iter(), cmod.as_ref());
        let ct_it = adapters::adapter_to_rid_and_content_type(c_it, md_analyzer);
        let s2t_idx: Vec<_> = adapters::adapter_to_link_src_2_tgt(ct_it, lrmod.as_ref()).collect();
        let elapsed = start.elapsed();
        debug!("Link and Backlink extraction: {:?}", elapsed);

        let start = Instant::now();
        let lmod = Rc::new(link::DefaultLinkModel::new(s2t_idx));
        let elapsed = start.elapsed();
        debug!("Creation of DefaultLinkModel: {:?}", elapsed);

        let start = Instant::now();
        let md_meta_data = zip(md_fs_meta_data.clone(), md_doc_meta_data).map(|f| {
            assert!(f.0 .0 == f.1 .0); // ensure that rids are the same.
            (f.0 .0, f.0 .1, f.1 .1)
        });
        let nmod = Rc::new(note::DefaultNoteModel::new(md_meta_data));
        let elapsed = start.elapsed();
        debug!("Creation of DefaultNoteModel: {:?}", elapsed);

        let start = Instant::now();
        let md_retriever_adapter =
            Rc::new(adapters::to_vault::NoteMetadataRetriever::new(nmod.clone()));
        let content_retriever_adapter = Rc::new(
            adapters::to_vault::MdContentRetrieverAdapter::new(cmod.clone()),
        );

        let get_backlinks_adapter = Rc::new(adapters::to_vault::GetBacklinksAdapter::new(
            lmod.clone(),
            rmod.clone(),
        ));

        let get_links_adapter = Rc::new(adapters::to_vault::GetLinksAdapter::new(
            lmod.clone(),
            rmod.clone(),
        ));
        let vrid_index =
            adapters::to_vault::convert_resource_ids_to_vault_format(nmod.create_iter());

        let vault = vault::VaultImpl::new(
            vrid_index,
            md_retriever_adapter,
            content_retriever_adapter,
            get_backlinks_adapter,
            get_links_adapter,
        );
        let elapsed = start.elapsed();
        debug!("Creation of Vault: {:?}", elapsed);

        let start = Instant::now();
        let content_retriever_adapter = Rc::new(
            adapters::to_note_writer::MdContentRetrieverAdapter::new(cmod.clone()),
        );

        let _note_writer = Rc::new(note_writer::NoteWriter::new(content_retriever_adapter));
        let elapsed = start.elapsed();
        debug!("Creation of NoteWriter: {:?}", elapsed);
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
    pub fn get_vault(&self) -> vault::VaultImpl {
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
