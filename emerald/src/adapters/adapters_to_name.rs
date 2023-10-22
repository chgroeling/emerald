use crate::types;
use crate::Result;

pub fn adapter_from_rid_to_name<'a>(
    it_src: impl IntoIterator<Item = &'a types::ResourceId>,
) -> Result<impl Iterator<Item = (&'a types::ResourceId, String)>> {
    // Assumption: All resource ids are encoded in utf8 nfc

    // Iterator yields (ResourceId, NameOfResourceId)
    let ret: Result<Vec<_>> = it_src
        .into_iter()
        .map(|resource_id| {
            let res_id_comp = resource_id.split()?;
            let name = res_id_comp.name.to_lowercase();
            Ok((resource_id, name))
        })
        .collect();

    match ret {
        Ok(vector) => Ok(vector.into_iter()),
        Err(err) => Err(err),
    }
}
