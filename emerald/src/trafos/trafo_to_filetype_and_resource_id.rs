use crate::{
    resources::meta_data_loader::MetaDataLoader,
    types::{meta_data::FileType, ResourceId},
};

pub fn trafo_to_filetype_and_resource_id<'a>(
    iter: impl Iterator<Item = ResourceId> + 'a,
    meta_data_loader: &'a impl MetaDataLoader,
) -> impl Iterator<Item = (FileType, ResourceId)> + 'a {
    iter.map(|f| {
        let res_meta_data = meta_data_loader.load(&f);
        if let Ok(meta_data) = res_meta_data {
            (meta_data.file_type, f.clone())
        } else {
            (FileType::NoFileType(), f.clone())
        }
    })
}
