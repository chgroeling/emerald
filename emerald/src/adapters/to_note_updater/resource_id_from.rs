use crate::model::note_updater;
use crate::types;

impl From<types::ResourceId> for note_updater::ExResourceId {
    fn from(value: types::ResourceId) -> Self {
        Self(value.0)
    }
}

impl From<note_updater::ExResourceId> for types::ResourceId {
    fn from(value: note_updater::ExResourceId) -> Self {
        Self(value.0)
    }
}
