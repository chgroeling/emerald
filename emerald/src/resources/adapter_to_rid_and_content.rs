use super::ContentLoader;
use crate::error::Result;
use crate::types;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn adapter_to_rid_and_content<'a>(
    it_src: impl IntoIterator<Item = &'a types::ResourceId>,
    content_loader: &'a impl ContentLoader,
) -> Result<impl Iterator<Item = (types::ResourceId, types::Content)>> {
    let ret: Result<Vec<_>> = it_src
        .into_iter()
        .map(|rid| -> Result<(types::ResourceId, types::Content)> {
            let content = content_loader.load(rid)?;
            trace!("Loaded {:?} into string", rid);
            Ok((rid.clone(), content))
        })
        .collect();

    match ret {
        Ok(vector) => Ok(vector.into_iter()),
        Err(err) => Err(err),
    }
}
