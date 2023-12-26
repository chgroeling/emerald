use super::{resource_object::ResourceObject, resource_object_translation::convert_ro_to_rid};
use crate::types;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::path::Path;

pub fn adapter_to_ro_and_rid<'a>(
    it_src: impl IntoIterator<Item = &'a ResourceObject> + 'a,
    common_path: &'a Path,
) -> impl Iterator<Item = (&ResourceObject, types::ResourceId)> + 'a {
    it_src
        .into_iter()
        .map(move |ro| (ro, convert_ro_to_rid(ro, common_path)))
}

#[cfg(test)]
mod tests {
    use super::adapter_to_ro_and_rid;
    use super::ResourceObject;
    use crate::types::ResourceId;
    use std::path::PathBuf;

    #[test]
    fn test_resolve_different_utf8_norm_match() {
        let ros: Vec<_> = vec![ResourceObject::File("testpäth".into())];
        let common_path: PathBuf = "".into();

        let res: Vec<_> = adapter_to_ro_and_rid(ros.iter(), &common_path).collect();

        assert_eq!(
            res,
            vec![(
                &ResourceObject::File("testpäth".into()),
                ResourceId("testpäth".into())
            )]
        );
    }

    #[test]
    fn test_resolve_with_different_utf8_norm_match_2() {
        let ros: Vec<_> = vec![ResourceObject::File("testpäth".into())];
        let common_path: PathBuf = "".into();

        let res: Vec<_> = adapter_to_ro_and_rid(ros.iter(), &common_path).collect();

        assert_eq!(
            res,
            vec![(
                &ResourceObject::File("testpäth".into()),
                ResourceId("testpäth".into())
            )]
        );
    }
}
