use super::{
    md_provider::MdProvider, timestamp_provider::TimestampProvider, title_provider::TitleProvider,
};

pub trait ProviderFactory {
    fn create_title_provider(&self) -> Box<dyn TitleProvider>;
    fn create_markdown_provider(&self) -> Box<dyn MdProvider>;
    fn create_created_provider(&self) -> Box<dyn TimestampProvider>;
    fn create_modified_provider(&self) -> Box<dyn TimestampProvider>;
}
