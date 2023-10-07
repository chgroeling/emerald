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
    pub ep_iter_src: Rc<T>,
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
