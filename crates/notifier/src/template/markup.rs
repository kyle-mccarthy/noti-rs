use super::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MarkupType {
    Mjml,
    Html,
}

pub enum Markup<'a> {
    Mjml(&'a str),
}

impl<'a> Markup<'a> {
    /// Parse the markup into a string
    pub fn parse(&self) -> Result<String, Error> {
        let output = match self {
            Self::Mjml(mjml) => mrml::mjml::MJML::parse(mjml)
                .map_err(|source| Error::Markup {
                    source: anyhow::Error::msg(source.to_string()),
                    ty: MarkupType::Mjml,
                })?
                .render(&mrml::prelude::render::Options::default())
                .map_err(|source| Error::Markup {
                    source: anyhow::Error::msg(source.to_string()),
                    ty: MarkupType::Mjml,
                })?,
        };

        Ok(output)
    }
}
