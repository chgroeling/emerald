use std::rc::Rc;

/// This struct holds a reference counted string
/// to hold content from markdown files.
pub struct Content(Rc<String>);