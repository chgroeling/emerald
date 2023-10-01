use super::{content_provider::ContentProvider, title_provider::TitleProvider};

pub trait ProviderFactory {
    fn create_title_provider(&self) -> Box<dyn TitleProvider>;
    fn create_content_provider(&self) -> Box<dyn ContentProvider>;
}
