use formatify::{Formatify, PlaceholderFormatter};

#[derive(Clone, Debug)]
pub enum FormatOptions {
    Overview,
    ShowMarkdown,
    Custom(String),
}

#[derive(Clone)]
pub struct FormatOptionParser;

impl clap::builder::TypedValueParser for FormatOptionParser {
    type Value = FormatOptions;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        if value.is_empty() {
            let err = clap::Error::new(clap::error::ErrorKind::ValueValidation).with_cmd(cmd);
            return Err(err);
        }
        let Some(ok_val) = value.to_str() else {
            let err = clap::Error::new(clap::error::ErrorKind::ValueValidation).with_cmd(cmd);
            return Err(err);
        };

        let ret = match ok_val {
            "overview" => FormatOptions::Overview,
            "all" => FormatOptions::ShowMarkdown,
            _ => {
                let custom_fmt = ok_val.to_owned();
                let expr_parser = Formatify::new();

                // # Determine which placeholders in the given format string are valid
                let placeholders = expr_parser.extract_placeholder_keys(&custom_fmt);

                // check if at least one placeholder can be found
                if placeholders.is_empty() {
                    let err =
                        clap::Error::new(clap::error::ErrorKind::ValueValidation).with_cmd(cmd);

                    return Err(err);
                }
                FormatOptions::Custom(custom_fmt)
            }
        };

        Ok(ret)
    }
}
