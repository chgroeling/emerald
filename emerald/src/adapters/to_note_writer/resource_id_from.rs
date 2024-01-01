use crate::model::note_writer;
use crate::types;

impl From<types::ResourceId> for note_writer::ExResourceId {
    fn from(value: types::ResourceId) -> Self {
        Self(value.0)
    }
}

impl From<note_writer::ExResourceId> for types::ResourceId {
    fn from(value: note_writer::ExResourceId) -> Self {
        Self(value.0)
    }
}
