#[derive(Debug, Clone)]

/// This struct holds a reference counted string
/// to hold content from markdown files.
pub struct Content(pub String);

impl From<&str> for Content {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
impl From<String> for Content {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<Content> for String {
    fn from(value: Content) -> Self {
        value.0.to_owned()
    }
}

impl From<&Content> for String {
    fn from(value: &Content) -> Self {
        value.0.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::Content;
    #[test]
    fn test_clone() {
        let s1: Content = "Hello world".into();
        let s2 = s1.clone();

        assert_eq!(String::from(s2), "Hello world"); // ensure that s2 is not deconstructed until this point
    }
}
