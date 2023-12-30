/// Represents a unique identifier (UID).
///
/// This struct encapsulates a UID as a boxed string for efficient storage.

#[derive(Debug, Clone, PartialEq, Hash, Default)]

pub struct Uid(pub Box<str>);

/// Enables equality comparisons for `Uid`.
///
/// `Uid` instances are considered equal if their inner boxed strings are equal.
impl Eq for Uid {}
