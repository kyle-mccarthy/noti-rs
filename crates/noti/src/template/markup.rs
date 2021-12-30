use super::Error;

#[derive(Debug)]
pub enum Markup<'a> {
    Text(&'a str),
    HTML(&'a str),
    MJML(&'a str),
}

impl<'a> Markup<'a> {
    /// Parse the markup into a string. Some types of markup need to parsed
    /// (MJML), while others can directly return their contents (Text).
    pub fn parse(&self) -> Result<String, Error> {
        match self {
            Self::Text(contents) | Self::HTML(contents) => Ok(contents.to_string()),
            Self::MJML(mjml) => mrml::parse(mjml)
                .map_err(|e| Error::Markup(anyhow::Error::msg(e.to_string())))
                .map(|parsed| parsed.to_string()),
        }
    }
}
