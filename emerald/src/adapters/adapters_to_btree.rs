use crate::types;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DocumentMetaData {
    tags: Option<String>,
    aliases: Option<Vec<String>>,
    created: Option<String>,
    modified: Option<String>,
    keywords: Option<Vec<String>>,
}

pub fn adapter_to_btree<'a>(
    it_src: impl IntoIterator<Item = (&'a types::ResourceId, &'a str)> + 'a,
) -> impl Iterator<Item = (&'a types::ResourceId, DocumentMetaData)> + 'a {
    let it = it_src.into_iter().filter_map(|f| {
        let res = serde_yaml::from_str::<DocumentMetaData>(f.1);
        match res {
            Ok(yaml_meta_data) => Some((f.0, yaml_meta_data)),
            Err(err) => {
                warn!("Invalid yaml found in {:?}\nError: {}\n{}", f.0, err, f.1);
                None
            }
        }
    });
    it
}
