use std::{any::TypeId, collections::HashMap};

use uuid::Uuid;

pub mod email;
pub mod other;

use self::email::{RegisteredEmailTemplate, RenderedEmailTemplate};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to parse the template: {0:?}")]
    Parser(String),

    #[error("Failed to parse the handlebars template: {reason}")]
    Template {
        reason: String,
        line_number: Option<usize>,
        column_number: Option<usize>,
        source: handlebars::TemplateError,
    },

    #[error("A template for `T` has not been registered")]
    UnknownTemplate,

    #[error("Failed to render the template: {description}")]
    Render {
        description: String,
        line_number: Option<usize>,
        column_number: Option<usize>,
        source: handlebars::RenderError,
    },
}

impl From<handlebars::TemplateError> for Error {
    fn from(source: handlebars::TemplateError) -> Self {
        let reason = source.reason.to_string();

        Self::Template {
            reason,
            line_number: source.line_no,
            column_number: source.column_no,
            source,
        }
    }
}

impl From<handlebars::RenderError> for Error {
    fn from(source: handlebars::RenderError) -> Self {
        let description = source.desc.to_string();

        Self::Render {
            description,
            line_number: source.line_no,
            column_number: source.column_no,
            source,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
/// ID for a registered template.
pub struct TemplateId(String);

impl Default for TemplateId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_hyphenated().to_string())
    }
}

impl AsRef<str> for TemplateId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TemplateId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub trait Template: private::Sealed {
    fn register(engine: &mut TemplateEngine) -> Result<(), Error>;
}

pub trait Register {
    type Template;

    fn register(&mut self) -> Result<(), Error>;
}

#[non_exhaustive]
enum RegisteredTemplate {
    Email(RegisteredEmailTemplate),
}

/// A template that has been rendered
#[non_exhaustive]
pub enum RenderedTemplate {
    /// A rendered EmailTemplate
    Email(RenderedEmailTemplate),
}

#[derive(Default)]
pub struct TemplateEngine<'a> {
    engine: handlebars::Handlebars<'a>,
    templates: HashMap<TypeId, RegisteredTemplate>,
}

impl<'a> TemplateEngine<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Render a template of type `T`. The content's of `T` are the payload/data provided to the
    /// template.
    pub fn render<T: Template>(&self, data: &T) -> Result<RenderedTemplate, Error> {
        let template = self
            .templates
            .get(&TypeId::of::<T>())
            .ok_or(Error::UnknownTemplate)?;

        match template {
            RegisteredTemplate::Email(template) => {
                let html = self.engine.render(template.html.as_ref(), &data)?;
                let subject = self.engine.render(template.subject.as_ref(), &data)?;

                let mut text = None;

                if let Some(text_id) = &template.text {
                    text = Some(self.engine.render(text_id.as_ref(), &data)?);
                }

                let rendered = RenderedEmailTemplate {
                    html,
                    text,
                    subject,
                };

                Ok(RenderedTemplate::Email(rendered))
            }
        }
    }
}

mod private {
    use serde::Serialize;

    use super::email::EmailTemplate;

    pub trait Sealed: std::any::Any + Serialize {}

    impl<T: EmailTemplate> Sealed for T {}
}
