use super::{markdown_provider::MarkdownProvider, title_provider::TitleProvider};

pub trait ProviderFactory {
    fn create_title_provider(&self) -> Box<dyn TitleProvider>;
    fn create_markdown_provider(&self) -> Box<dyn MarkdownProvider>;
}
