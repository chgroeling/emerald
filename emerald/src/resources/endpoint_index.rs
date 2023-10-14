use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::Result;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::types::EndPoint;

use EndPoint::*;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct EndpointIndex {
    endpoint_list: Rc<Vec<EndPoint>>,
}

pub fn trafo_pathes_to_endpoints(file_list: Vec<PathBuf>) -> impl Iterator<Item = EndPoint> {
    let mut endpoint_list = Vec::<EndPoint>::new();
    for file_path in file_list {
        let endpoint = if file_path
            .extension()
            .is_some_and(|ext| ext == "md" || ext == "markdown")
        {
            FileMarkdown(file_path)
        } else {
            FileUnknown(file_path)
        };

        endpoint_list.push(endpoint);
    }

    endpoint_list.into_iter()
}

pub fn get_file_list_recursive(path: &Path) -> Result<Vec<PathBuf>> {
    trace!("get_file_list of path: {:?}", path);

    // get iterator for the actual directory
    let iter_dir = fs::read_dir(path)?;
    let mut file_list = Vec::<PathBuf>::new();

    for i in iter_dir {
        let iter_path = i?.path();

        if iter_path.is_dir() {
            let mut res = get_file_list_recursive(&iter_path)?;
            file_list.append(&mut res);
        } else {
            trace!("Append {:?} to file_list", &iter_path);

            file_list.push(iter_path);
        }
    }

    Ok(file_list)
}
