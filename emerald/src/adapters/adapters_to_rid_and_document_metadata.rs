use crate::types::{self, DocumentMetadata};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn adapter_to_rid_and_document_metadata<'a>(
    it_src: impl IntoIterator<Item = (types::ResourceId, &'a str)> + 'a,
) -> impl Iterator<Item = (types::ResourceId, types::DocumentMetadata)> + 'a {
    it_src.into_iter().map(|f| {
        let res = serde_yaml::from_str::<types::DocumentMetadata>(f.1);
        match res {
            Ok(yaml_meta_data) => (f.0, yaml_meta_data),
            Err(err) => {
                warn!("Invalid yaml found in {:?}\nError: {}\n{}", f.0, err, f.1);
                (f.0, DocumentMetadata::default())
            }
        }
    })
}
