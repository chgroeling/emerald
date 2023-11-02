use super::{
    md_provider::MdProvider, string_provider::StringProvider, timestamp_provider::TimestampProvider,
};

pub trait ProviderFactory {
    fn create_title_provider(&self) -> Box<dyn StringProvider>;
    fn create_markdown_provider(&self) -> Box<dyn MdProvider>;
    fn create_timestamp_created_provider(&self) -> Box<dyn TimestampProvider>;
    fn create_timestamp_modified_provider(&self) -> Box<dyn TimestampProvider>;
}
