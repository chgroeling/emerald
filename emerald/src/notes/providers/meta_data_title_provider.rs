use super::title_provider::TitleProvider;
use crate::{resources, types};

pub struct MetaDataTitleProvider<I>
where
    I: resources::MetaDataLoader,
{
    meta_data_loader: I,
}

impl<I> MetaDataTitleProvider<I>
where
    I: resources::MetaDataLoader,
{
    pub fn new(meta_data_loader: I) -> Self {
        Self { meta_data_loader }
    }
}
impl<I> TitleProvider for MetaDataTitleProvider<I>
where
    I: resources::MetaDataLoader,
{
    fn get_title(&self, rid: &types::ResourceId) -> String {
        let meta_data = self.meta_data_loader.load(rid).unwrap();
        meta_data.file_stem
    }
}
