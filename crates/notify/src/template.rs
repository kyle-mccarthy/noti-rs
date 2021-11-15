use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use serde::Serialize;
use uuid::Uuid;

pub mod email;

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

pub trait Template: Any + Serialize {
    fn register(engine: &mut TemplateManager) -> Result<(), Error>;
}

pub trait Register {
    type Template;

    fn register(&mut self) -> Result<(), Error>;
}

#[non_exhaustive]
pub enum RegisteredTemplate {
    Email(RegisteredEmailTemplate),
}

impl RegisteredTemplate {
    pub fn unregister(self, manager: &mut TemplateManager) {
        match self {
            Self::Email(email_template) => {
                manager
                    .engine
                    .unregister_template(email_template.html.as_ref());
                manager
                    .engine
                    .unregister_template(email_template.subject.as_ref());

                if let Some(text_id) = email_template.text {
                    manager.engine.unregister_template(text_id.as_ref());
                }
            }
        }
    }
}

/// A template that has been rendered
#[non_exhaustive]
pub enum RenderedTemplate {
    /// A rendered EmailTemplate
    Email(RenderedEmailTemplate),
}

impl RenderedTemplate {
    pub fn as_email(&self) -> Option<&RenderedEmailTemplate> {
        match self {
            Self::Email(email) => Some(email),
        }
    }

    pub fn into_email(self) -> Option<RenderedEmailTemplate> {
        match self {
            Self::Email(email) => Some(email),
        }
    }
}

#[derive(Default)]
pub struct TemplateManager<'a> {
    engine: handlebars::Handlebars<'a>,
    templates: HashMap<TypeId, RegisteredTemplate>,
}

impl<'a> TemplateManager<'a> {
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

    /// Register a template with the template manager
    pub fn register<T: Template>(&mut self) -> Result<(), Error> {
        T::register(self)
    }
}
