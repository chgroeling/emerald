use crate::types;

/// Represents a location of a resource.
///
/// This struct holds a resource identifier, along with normalized file name
/// and directory path information.
///
/// # Fields
///
/// * `rid`: Resource identifier (`types::ResourceId`).
/// * `norm_filename`: Normalized file name (`Box<str>`).
/// * `dir_path`: Path where the file resides (`Box<str>`).
pub struct ResourceLoc {
    pub rid: types::ResourceId,
    pub norm_filename: Box<str>,
    pub dir_path: Box<str>,
}
