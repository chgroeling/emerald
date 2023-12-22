use std::collections::BTreeMap;

use crate::types;

pub fn adapter_to_btree<'a>(
    it_src: impl IntoIterator<Item = (&'a types::ResourceId, &'a str)> + 'a,
) -> impl Iterator<Item = (&'a types::ResourceId, BTreeMap<String, String>)> + 'a {
    std::iter::empty::<(&'a types::ResourceId, BTreeMap<String, String>)>()
}
