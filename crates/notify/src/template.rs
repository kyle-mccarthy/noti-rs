use crate::{channel::ChannelType, message::email::EmailContents};

pub mod email;
pub mod manager;

use manager::Manager;
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error while running channel specific parser")]
    PreParse(anyhow::Error),

    #[error("Error while parsing the template: {0:?}")]
    Parse(#[from] handlebars::TemplateError),

    #[error("Failed to render the template: {0:?}")]
    Render(#[from] handlebars::RenderError),
}

pub trait Template: sealed::Sealed {
    fn channel(&self) -> ChannelType;

    fn register(&self, manager: &mut Manager) -> Result<RegisteredTemplate, Error>;
}

pub trait Render: sealed::Sealed {
    fn render<T: Serialize>(
        &self,
        manager: &mut Manager,
        data: &T,
    ) -> Result<RenderedTemplate, Error>;
}

pub struct TemplateId(String);

impl TemplateId {
    pub fn new() -> Self {
        let id = uuid::Uuid::new_v4().to_simple().to_string();
        Self(id)
    }
}

impl Default for TemplateId {
    fn default() -> Self {
        Self::new()
    }
}

pub enum RegisteredTemplate {
    Email(email::RegisteredEmailTemplate),
}

impl sealed::Sealed for RegisteredTemplate {}

impl Render for RegisteredTemplate {
    fn render<T: Serialize>(
        &self,
        manager: &mut Manager,
        data: &T,
    ) -> Result<RenderedTemplate, Error> {
        match self {
            Self::Email(tmpl) => tmpl.render(manager, data),
        }
    }
}

pub enum RenderedTemplate {
    Email(EmailContents),
}

impl RenderedTemplate {
    pub fn is_email(&self) -> bool {
        matches!(self, Self::Email(_))
    }

    pub fn as_email(&self) -> Option<&EmailContents> {
        match self {
            Self::Email(email) => Some(email),
        }
    }

    pub fn into_email(self) -> Option<EmailContents> {
        match self {
            Self::Email(email) => Some(email),
        }
    }
}

mod sealed {
    pub trait Sealed {}
}
