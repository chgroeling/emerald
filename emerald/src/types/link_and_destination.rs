use super::{Link, ResourceId};

// Structs holds a Link and its destination Resource Id if existant.
pub type LinkAndDestination = (Link, Option<ResourceId>);
