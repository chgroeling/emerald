use crate::Result;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::fs;
use std::path::{Path, PathBuf};

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
