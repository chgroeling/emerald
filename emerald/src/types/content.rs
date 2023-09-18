use std::rc::Rc;

/// This struct holds a reference counted string
/// to hold content from markdown files.
pub struct Content(pub Rc<String>);

impl From<&str> for Content {
    fn from(value: &str) -> Self {
        Self(Rc::new(value.to_string()))
    }
}
impl From<String> for Content {
    fn from(value: String) -> Self {
        Self(Rc::new(value))
    }
}
