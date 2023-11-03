use super::provider::Provider;

pub trait ProviderFactory {
    fn create_title_provider(&self) -> Box<dyn Provider<String>>;
    fn create_markdown_provider(&self) -> Box<dyn Provider<String>>;
    fn create_size_provider(&self) -> Box<dyn Provider<u64>>;
    fn create_created_time_provider(&self) -> Box<dyn Provider<i64>>;
    fn create_modified_time_provider(&self) -> Box<dyn Provider<i64>>;
}
