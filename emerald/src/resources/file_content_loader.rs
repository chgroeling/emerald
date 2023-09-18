use std::{collections::HashMap, fs, path::Path};

use crate::indexes::AllEndpointsIterSource;
use crate::types::Content;
use crate::types::EndPoint;
use crate::types::ResourceId;
use crate::utils::endpoint_translation::convert_endpoint_to_resource_id;
use crate::EmeraldError;
use crate::Result;

use EmeraldError::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::content_loader::ContentLoader;

pub type ResourceIdToEndpoint = HashMap<ResourceId, EndPoint>;

pub struct FileContentLoader {
    resource_id_to_endpoint: ResourceIdToEndpoint,
}

impl FileContentLoader {
    pub fn new(endpoint_iter_src: &impl AllEndpointsIterSource, common_path: &Path) -> Self {
        let mut resource_id_to_endpoint: ResourceIdToEndpoint = ResourceIdToEndpoint::new();

        for endpoint in endpoint_iter_src.all_iter() {
            let opt_resource_id = convert_endpoint_to_resource_id(endpoint.clone(), common_path);

            // TODO: ONLY INCLUDE MARKDOWN FILES
            if let Some(resource_id) = opt_resource_id {
                resource_id_to_endpoint.insert(resource_id, endpoint);
            } else {
                warn!("Can't convert Endpoint '{:?}' to ResourceId.", &endpoint);
            }
        }

        Self {
            resource_id_to_endpoint,
        }
    }

    #[inline]
    fn get(&self, resource_id: &ResourceId) -> Result<EndPoint> {
        match self.resource_id_to_endpoint.get(resource_id) {
            Some(result) => Ok(result.clone()),
            None => Err(EndPointNotFound),
        }
    }
}

impl ContentLoader for FileContentLoader {
    fn load(&self, resource_id: &ResourceId) -> Result<Content> {
        let endpoint = self.get(resource_id)?;

        let EndPoint::FileMarkdown(md_path) = endpoint else {
            return Err(NotAMarkdownFile);
        };
        Ok(fs::read_to_string(md_path)?.into())
    }
}

#[cfg(test)]
mod tests {
    use super::{AllEndpointsIterSource, EmeraldError, EndPoint, FileContentLoader};
    use std::path::PathBuf;
    use EmeraldError::*;
    use EndPoint::*;

    struct MockEndPointIndex {
        endpoints: Vec<EndPoint>,
    }

    impl AllEndpointsIterSource for MockEndPointIndex {
        type Iter = std::vec::IntoIter<EndPoint>;
        fn all_iter(&self) -> Self::Iter {
            self.endpoints.clone().into_iter()
        }
    }

    #[test]
    fn test_resolve_single() {
        let common_path = PathBuf::from("");
        let mock = MockEndPointIndex {
            endpoints: vec![File("testpath".into())],
        };

        let dut = FileContentLoader::new(&mock, &common_path);
        let ep = dut.get(&"[[testpath]]".into()).unwrap();

        assert!(matches!(ep, EndPoint::File(path) if path==PathBuf::from("testpath")));
    }

    #[test]
    fn test_resolve_single_with_different_utf8_norm_match() {
        let common_path = PathBuf::from("");
        let mock = MockEndPointIndex {
            endpoints: vec![File("testpäth".into())],
        };
        let dut = FileContentLoader::new(&mock, &common_path);
        let ep = dut.get(&"[[testpäth]]".into()).unwrap();

        assert!(matches!(ep, EndPoint::File(path) if path==PathBuf::from("testpäth")));
    }

    #[test]
    fn test_resolve_single_with_different_utf8_norm_fail() {
        let common_path = PathBuf::from("");
        let mock = MockEndPointIndex {
            endpoints: vec![File("testpäth".into())],
        };
        let dut = FileContentLoader::new(&mock, &common_path);
        let ep = dut.get(&"[[testpäth]]".into());

        assert!(matches!(ep, Err(EndPointNotFound)));
    }
}
