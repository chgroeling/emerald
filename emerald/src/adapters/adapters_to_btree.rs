use crate::types;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::collections::BTreeMap;

pub fn adapter_to_btree<'a>(
    it_src: impl IntoIterator<Item = (&'a types::ResourceId, &'a str)> + 'a,
) -> impl Iterator<Item = (&'a types::ResourceId, BTreeMap<String, String>)> + 'a {
    let it = it_src.into_iter().filter_map(|f| {
        let res = serde_yaml::from_str::<BTreeMap<String, String>>(f.1);
        match res {
            Ok(yaml_meta_data) => Some(yaml_meta_data),
            Err(err) => {
                warn!("Invalid yaml found in {:?}\nError: {}\n{}", f.0, err, f.1);
                None
            }
        }
    });

    for i in it {
        //   println!("{:?}", i)
    }
    std::iter::empty::<(&'a types::ResourceId, BTreeMap<String, String>)>()
}
