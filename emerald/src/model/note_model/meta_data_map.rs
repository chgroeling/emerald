use crate::types;
use std::{
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
};

use super::meta_data_retriever::MetaDataRetriever;

#[derive(Clone)]
pub struct MetaDataMap {
    meta_data_map: Rc<HashMap<types::ResourceId, types::MetaData>>,
}

impl MetaDataMap {
    pub fn new<'a>(
        it_src: impl IntoIterator<Item = (&'a types::ResourceId, &'a types::MetaData)>,
    ) -> Self {
        let mut meta_data_map = HashMap::<types::ResourceId, types::MetaData>::new();
        for (rid, meta_data) in it_src.into_iter() {
            match meta_data_map.entry(rid.clone()) {
                Entry::Occupied(mut _e) => {
                    panic!("No duplicate entries allowed")
                }
                Entry::Vacant(e) => {
                    e.insert(meta_data.clone());
                }
            }
        }
        Self {
            meta_data_map: Rc::new(meta_data_map),
        }
    }
}

impl MetaDataRetriever for MetaDataMap {
    fn retrieve(&self, md: types::ResourceId) -> &types::MetaData {
        // Option is not returned because meta data should be consistent at this point
        self.meta_data_map
            .get(&md)
            .expect("Meta data was not stored. This should not happen")
    }
}
