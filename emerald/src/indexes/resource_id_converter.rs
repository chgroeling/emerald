use std::rc::Rc;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::{
    resources::{endpoints_iter_src::EndpointsIterSrc, resource_id_resolver::ResourceIdResolver},
    types::{EndPoint, ResourceId},
};

use super::ResourceIdsIterSrc;

pub struct ResourceIdIterator<T, U>
where
    T: Iterator<Item = EndPoint>,
    U: ResourceIdResolver,
{
    ep_iter: T,
    resource_id_resolver: Rc<U>,
}

impl<T, U> Iterator for ResourceIdIterator<T, U>
where
    T: Iterator<Item = EndPoint>,
    U: ResourceIdResolver,
{
    type Item = ResourceId;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let ep = self.ep_iter.next()?;
            let opt_resource_id = self.resource_id_resolver.resolve(&ep);
            if let Ok(resource_id) = opt_resource_id {
                return Some(resource_id);
            }

            warn!(
                "Obtaining resource id for endpoint {:?} yielded {:?} ",
                ep, opt_resource_id
            );
        }
    }
}

pub struct ResourceIdConverter<T, U>
where
    T: EndpointsIterSrc,
    U: ResourceIdResolver,
{
    pub ep_iter_src: T,
    pub resource_id_resolver: Rc<U>,
}

impl<T, U> ResourceIdsIterSrc for ResourceIdConverter<T, U>
where
    T: EndpointsIterSrc,
    U: ResourceIdResolver,
{
    type Iter = ResourceIdIterator<T::Iter, U>;

    fn iter(&self) -> Self::Iter {
        let ep_iter = self.ep_iter_src.iter();
        ResourceIdIterator {
            ep_iter,
            resource_id_resolver: self.resource_id_resolver.clone(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{EndPoint, ResourceId, ResourceIdConverter};
    use crate::indexes::ResourceIdsIterSrc;
    use crate::resources::endpoints_iter_src::MockEndpointsIterSrc;
    use crate::resources::resource_id_resolver::MockResourceIdResolver;
    use std::iter::zip;
    use std::rc::Rc;
    use EndPoint::*;

    fn create_dut_everything_matches(
        test_ep_list: Vec<EndPoint>,
        iter_res: Vec<ResourceId>,
    ) -> ResourceIdConverter<MockEndpointsIterSrc, MockResourceIdResolver> {
        let mut mock_it_src = MockEndpointsIterSrc::new();
        let mut mock_res_id_res = MockResourceIdResolver::new();

        mock_it_src
            .expect_iter()
            .return_const(test_ep_list.clone().into_iter());

        // iterate test data to set expectations and returns for resolver
        for (ep, resource_id) in zip(test_ep_list, iter_res) {
            mock_res_id_res
                .expect_resolve()
                .withf(move |f| f == &ep)
                .returning(move |_f| Ok(resource_id.clone()));
        }

        ResourceIdConverter {
            ep_iter_src: mock_it_src,
            resource_id_resolver: Rc::new(mock_res_id_res),
        }
    }

    #[test]
    fn test_iter_empty() {
        let dut = create_dut_everything_matches(vec![], vec![]);
        let result: Vec<ResourceId> = dut.iter().collect();
        let expected: Vec<ResourceId> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iter_one() {
        let dut = create_dut_everything_matches(
            vec![FileUnknown("testpath".into())],
            vec![ResourceId("[[rid1]]".into())],
        );
        let result: Vec<ResourceId> = dut.iter().collect();
        let expected: Vec<ResourceId> = vec!["[[rid1]]".into()];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_iter_two() {
        let dut = create_dut_everything_matches(
            vec![
                FileUnknown("test_file1".into()),
                FileUnknown("test_file2".into()),
            ],
            vec![ResourceId("[[rid1]]".into()), ResourceId("[[rid2]]".into())],
        );

        let result: Vec<ResourceId> = dut.iter().collect();
        let expected: Vec<ResourceId> = vec!["[[rid1]]".into(), "[[rid2]]".into()];
        assert_eq!(result, expected);
    }
}
