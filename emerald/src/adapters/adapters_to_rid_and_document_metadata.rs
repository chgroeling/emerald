use crate::types::{self, DocumentMetadata};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn adapter_to_rid_and_document_metadata<'a>(
    it_src: impl IntoIterator<Item = (types::ResourceId, Option<&'a str>)> + 'a,
) -> impl Iterator<Item = (types::ResourceId, types::DocumentMetadata)> + 'a {
    it_src.into_iter().map(|f| {
        if let Some(yaml) = f.1 {
            let res = serde_yaml::from_str::<types::DocumentMetadata>(yaml);
            match res {
                Ok(yaml_meta_data) => (f.0, yaml_meta_data),
                Err(err) => {
                    warn!("Invalid yaml found in {:?}\nError: {}\n{}", f.0, err, yaml);
                    (f.0, DocumentMetadata::default())
                }
            }
        } else {
            (f.0, DocumentMetadata::default())
        }
    })
}
