use super::title_provider::TitleProvider;

pub trait ProviderFactory {
    fn create_title_provider(&self) -> Box<dyn TitleProvider>;
}
