use crate::types;

pub struct ResourceLoc {
    pub rid: types::ResourceId,
    pub norm_filename: Box<str>,
    pub dir_path: Box<str>,
}
