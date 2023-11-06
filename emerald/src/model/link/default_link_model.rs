use super::links_iter_src::LinksIterSrc;
use super::src_iter_retriever::SrcIterRetriever;
use super::src_links_map::SrcLinksMap;
use super::tgt_iter_retriever::TgtIterRetriever;
use super::tgt_links_map::TgtLinksMap;
use crate::types;

pub struct DefaultLinkModel {
    link_index: Vec<types::LinkSrc2Tgt>,
    src_links_map: SrcLinksMap,
    tgt_links_map: TgtLinksMap,
}

impl DefaultLinkModel {
    pub fn new(it_links_src_2_tgt: impl IntoIterator<Item = types::LinkSrc2Tgt>) -> Self {
        let link_index: Vec<_> = it_links_src_2_tgt.into_iter().collect();
        let src_links_map = SrcLinksMap::new(link_index.iter());
        let tgt_links_map = TgtLinksMap::new(link_index.iter());

        Self {
            link_index,
            src_links_map,
            tgt_links_map,
        }
    }
}

impl TgtIterRetriever for DefaultLinkModel {
    fn retrieve(
        &self,
        src: &types::ResourceId,
    ) -> Option<Box<dyn Iterator<Item = types::Link2Tgt>>> {
        self.tgt_links_map.retrieve(src)
    }
}
impl SrcIterRetriever for DefaultLinkModel {
    fn retrieve(
        &self,
        src: &types::ResourceId,
    ) -> Option<Box<dyn Iterator<Item = types::LinkFrmSrc>>> {
        self.src_links_map.retrieve(src)
    }
}

impl LinksIterSrc for DefaultLinkModel {
    type Iter = std::vec::IntoIter<types::LinkSrc2Tgt>;

    fn create_iter(&self) -> Self::Iter {
        self.link_index.clone().into_iter()
    }
}
