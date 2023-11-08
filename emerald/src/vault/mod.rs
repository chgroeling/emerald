mod get_backlinks;
mod get_backlinks_impl;
mod get_links;
mod get_links_impl;
mod link_query_result;
mod note;
mod note_factory;
mod note_factory_impl;
mod note_types;
mod resource_ref;
mod timestamp;
mod vault_impl;
mod vault_trait;

pub use note::Note;
pub use note_factory::NoteFactory;
pub use note_factory_impl::NoteFactoryImpl;
pub use vault_impl::VaultImpl;
pub use vault_trait::Vault;
