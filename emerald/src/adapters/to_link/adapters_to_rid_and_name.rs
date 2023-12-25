use crate::error::Result;
use crate::types;

pub fn adapter_to_rid_and_name<'a>(
    it_src: impl IntoIterator<Item = &'a (types::ResourceId, types::FilesystemMetadata)>,
) -> Result<impl Iterator<Item = (types::ResourceId, String)>> {
    // Assumption: All resource ids are encoded in utf8 nfc

    // Iterator yields (ResourceId, NameOfResourceId)
    let ret: Result<Vec<_>> = it_src
        .into_iter()
        .map(|(rid, fs_metadata)| {
            let name = fs_metadata.name.to_lowercase();
            Ok((rid.clone(), name))
        })
        .collect();

    match ret {
        Ok(vector) => Ok(vector.into_iter()),
        Err(err) => Err(err),
    }
}
