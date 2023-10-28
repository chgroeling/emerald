mod file_model;
mod link_model;
mod note_model;

pub use file_model::DefaultFileModel;
pub use file_model::FileCount;
pub use file_model::FilesIterSrc;

pub use note_model::DefaultNoteModel;
pub use note_model::LinksIterSrc;
pub use note_model::MetaDataRetriever;
pub use note_model::NoteCount;
pub use note_model::NotesIterSrc;

pub use link_model::DefaultLinkModel;
pub use link_model::ResourceIdResolver;

#[cfg(test)]
pub use note_model::MockLinksIterSrc;
