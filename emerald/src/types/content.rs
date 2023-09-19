use std::rc::Rc;

#[derive(Debug)]

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

impl Clone for Content {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::Content;
    #[test]
    fn test_clone() {
        let s1: Content = "Hello world".into();
        let s2 = s1.clone();

        assert_eq!(Rc::strong_count(&s1.0), 2);
        assert_eq!(*s2.0, "Hello world"); // ensure that s2 is not deconstructed until this point
    }
}
