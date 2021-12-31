use std::fmt::Display;

use mrml::prelude::render::Options as MrmlOptions;

use super::Error;

#[derive(Debug)]
pub enum Markup<'a> {
    Text(&'a str),
    Html(&'a str),
    Mjml(&'a str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MarkupType {
    Text,
    Html,
    Mjml,
}

impl Display for MarkupType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text => f.write_str("Text"),
            Self::Html => f.write_str("HTML"),
            Self::Mjml => f.write_str("MJML"),
        }
    }
}

impl<'a> Markup<'a> {
    /// Parse the markup into a string. Some types of markup need to parsed
    /// (MJML), while others can directly return their contents (Text).
    pub fn parse(&self) -> Result<String, Error> {
        match self {
            Self::Text(contents) | Self::Html(contents) => Ok(contents.to_string()),
            Self::Mjml(mjml) => {
                let parsed = mrml::parse(mjml).map_err(|e| Error::Markup {
                    source: anyhow::Error::msg(e.to_string()),

                    markup_type: MarkupType::Mjml,
                    context: Some("failed to parse the MJML"),
                })?;

                let rendered =
                    parsed
                        .render(&MrmlOptions::default())
                        .map_err(|e| Error::Markup {
                            source: anyhow::Error::msg(e.to_string()),
                            markup_type: MarkupType::Mjml,
                            context: Some("failed to render the MJML"),
                        })?;

                Ok(rendered)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_parse_mjml_markup() {
        let contents = indoc! {r#"
            <mjml>
                <mj-body>
                    <mj-section>
                        <mj-column>
                            <mj-text>
                                Hello World!
                            </mj-text>
                        </mj-column>
                    </mj-section>
                </mj-body>
            </mjml>
        "#};

        let template = Markup::Mjml(contents);
        let output = template.parse().unwrap();

        let expected = mrml::parse(&contents)
            .unwrap()
            .render(&MrmlOptions::default())
            .unwrap();

        // fairly silly assertion, but mjml -> html produces a ton of HTML relative to
        // the input + it also may make more sense to just check that our
        // template matches whatever mrml spits out?.
        pretty_assertions::assert_eq!(expected, output);
    }
}
