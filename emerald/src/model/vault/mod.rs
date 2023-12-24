mod document_metadata_from;
mod filesystem_metadata_from;
mod get_backlinks;
mod get_backlinks_impl;
mod get_links;
mod get_links_impl;
mod link_query_result;
mod link_query_result_builder;
mod note;
mod note_factory;
mod note_factory_impl;
mod note_metadata_retriever;
mod note_types;
mod resource_id;
mod resource_ref;
mod vault_impl;
mod vault_trait;

pub use note::Note;
pub use note_factory::NoteFactory;
pub use note_factory_impl::NoteFactoryImpl;
pub use note_types::NoteTypes;
pub use vault_impl::VaultImpl;
pub use vault_trait::Vault;

pub use note::DocumentMetadata;
pub use note::FilesystemMetadata;
pub use note_metadata_retriever::NoteMetadataRetriever;
pub use resource_id::ResourceId;