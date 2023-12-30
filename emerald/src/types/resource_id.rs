#[derive(Debug, Clone, PartialEq, Hash)]

/// Represents a unique identifier for a resource.
///
/// This struct is primarily used to represent resource identifiers as strings,
/// specifically as paths in the filesystem.
pub struct ResourceId(pub Box<str>);

/// Implements `From<&str>` for `ResourceId`, allowing conversion from string slice.
impl From<&str> for ResourceId {
    fn from(value: &str) -> Self {
        Self(value.to_owned().into_boxed_str())
    }
}

/// Implements `From<String>` for `ResourceId`, allowing conversion from `String`.
impl From<String> for ResourceId {
    fn from(value: String) -> Self {
        Self(value.into_boxed_str())
    }
}

/// Implements `From<Box<str>>` for `ResourceId`, allowing conversion from `Box<str>`.
impl From<Box<str>> for ResourceId {
    fn from(value: Box<str>) -> Self {
        Self(value)
    }
}

/// Enables equality comparisons for `ResourceId`.
///
/// `ResourceId` instances are considered equal if their inner boxed strings are equal.
impl Eq for ResourceId {}
