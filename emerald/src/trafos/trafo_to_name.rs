use crate::types::ResourceId;

pub fn trafo_from_res_id_to_name<'a>(
    resource_ids_iter: impl Iterator<Item = &'a ResourceId>,
) -> impl Iterator<Item = (&'a ResourceId, String)> {
    // Assumption: All resource ids are encoded in utf8 nfc

    // Iterator yields (ResourceId, NameOfResourceId)
    resource_ids_iter.map(|resource_id| {
        let res_id_comp = resource_id.split().unwrap();
        let name = res_id_comp.name.to_lowercase();
        (resource_id, name)
    })
}
