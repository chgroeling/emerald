use std::fmt::Display;

#[derive(Debug)]
pub struct LinkComps {
    pub path: Option<String>,
    pub name: String,
    pub label: Option<String>,
    pub section: Option<String>,
    pub anchor: Option<String>,
}

impl LinkComps {
    pub fn new(
        name: String,
        path: Option<String>,
        label: Option<String>,
        section: Option<String>,
        anchor: Option<String>,
    ) -> Self {
        LinkComps {
            path,
            name,
            label,
            section,
            anchor,
        }
    }

    #[allow(dead_code)]
    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    pub fn new_bare_link(name: String) -> Self {
        LinkComps {
            path: None,
            name,
            label: None,
            section: None,
            anchor: None,
        }
    }

    #[allow(dead_code)]
    pub fn new_link_with_path(name: String, path: String) -> Self {
        LinkComps {
            path: Some(path),
            name,
            label: None,
            section: None,
            anchor: None,
        }
    }
}

impl Display for LinkComps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[[")?;

        if let Some(path) = &self.path {
            write!(f, "{}/", path)?;
        }

        write!(f, "{}", self.name)?;

        if let Some(label) = &self.label {
            write!(f, "|{}", label)?
        }

        if let Some(section) = &self.section {
            write!(f, "#{}", section)?
        }

        if let Some(anchor) = &self.anchor {
            write!(f, "^{}", anchor)?
        }

        write!(f, "]]")
    }
}

impl From<&'static str> for LinkComps {
    fn from(value: &'static str) -> Self {
        Self::new_bare_link(value.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use crate::types::link_comps::LinkComps;

    #[test]
    fn test_fmt_bare_link() {
        let dut = LinkComps::new_bare_link("my_link".into());

        let res = dut.to_string();

        assert_eq!(res, "[[my_link]]")
    }

    #[test]
    fn test_fmt_link_with_path() {
        let dut = LinkComps::new_link_with_path("my_link".into(), "a/b/c".into());

        let res = dut.to_string();

        assert_eq!(res, "[[a/b/c/my_link]]")
    }

    #[test]
    fn test_fmt_link_with_path_and_label() {
        let dut = LinkComps::new(
            "my_link".into(),
            Some("a/b/c".into()),
            Some("my_label".into()),
            None,
            None,
        );

        let res = dut.to_string();

        assert_eq!(res, "[[a/b/c/my_link|my_label]]")
    }

    #[test]
    fn test_fmt_link_with_label() {
        let dut = LinkComps::new("my_link".into(), None, Some("my_label".into()), None, None);

        let res = dut.to_string();

        assert_eq!(res, "[[my_link|my_label]]")
    }

    #[test]
    fn test_fmt_link_with_label_and_section() {
        let dut = LinkComps::new(
            "my_link".into(),
            None,
            Some("my_label".into()),
            Some("my_section".into()),
            None,
        );

        let res = dut.to_string();

        assert_eq!(res, "[[my_link|my_label#my_section]]")
    }

    #[test]
    fn test_fmt_link_full() {
        let dut = LinkComps::new(
            "my_link".into(),
            Some("a/b/c".into()),
            Some("my_label".into()),
            Some("my_section".into()),
            Some("my_anchor".into()),
        );

        let res = dut.to_string();

        assert_eq!(res, "[[a/b/c/my_link|my_label#my_section^my_anchor]]")
    }
}
