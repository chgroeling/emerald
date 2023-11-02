use super::{string_provider::StringProvider, timestamp_provider::TimestampProvider};

pub trait ProviderFactory {
    fn create_title_provider(&self) -> Box<dyn StringProvider>;
    fn create_markdown_provider(&self) -> Box<dyn StringProvider>;
    fn create_created_time_provider(&self) -> Box<dyn TimestampProvider>;
    fn create_modified_time_provider(&self) -> Box<dyn TimestampProvider>;
}
