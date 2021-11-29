use serde::Serialize;

use super::{
    manager::Manager, Error, RegisteredTemplate, Render, RenderedTemplate, Template, TemplateId,
};
use crate::{channel::ChannelType, message::email::EmailContents};

pub struct EmailTemplate<'a> {
    pub subject: &'a str,
    pub html: &'a str,
    pub text: Option<&'a str>,
}

impl<'a> super::sealed::Sealed for EmailTemplate<'a> {}

impl<'a> Template for EmailTemplate<'a> {
    fn channel(&self) -> ChannelType {
        ChannelType::Email
    }

    fn register(&self, manager: &mut Manager) -> Result<RegisteredTemplate, Error> {
        let subject = manager.register(self.subject)?;

        let html = mrml::parse(self.html)
            .map_err(|e| Error::PreParse(anyhow::Error::msg(e.to_string())))?
            .to_string();
        let html = manager.register(&html)?;

        let text = if let Some(text) = &self.text {
            Some(manager.register(text)?)
        } else {
            None
        };

        Ok(RegisteredTemplate::Email(RegisteredEmailTemplate {
            subject,
            html,
            text,
        }))
    }
}

pub struct RegisteredEmailTemplate {
    subject: TemplateId,
    html: TemplateId,
    text: Option<TemplateId>,
}

impl super::sealed::Sealed for RegisteredEmailTemplate {}

impl Render for RegisteredEmailTemplate {
    fn render<T: Serialize>(
        &self,
        manager: &Manager,
        data: &T,
    ) -> Result<super::RenderedTemplate, Error> {
        let subject = manager.render(&self.subject, data)?;
        let html = manager.render(&self.html, data)?;
        let text = if let Some(text) = &self.text {
            Some(manager.render(text, data)?)
        } else {
            None
        };

        Ok(RenderedTemplate::Email(EmailContents {
            subject,
            html,
            text,
        }))
    }
}

pub struct RenderedEmailTemplate {
    subject: String,
    html: String,
    text: Option<String>,
}

impl RenderedEmailTemplate {
    /// Get a reference to the rendered email template's html.
    pub fn html(&self) -> &str {
        &self.html
    }

    /// Get a reference to the rendered email template's subject.
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Get a reference to the rendered email template's text.
    pub fn text(&self) -> Option<&String> {
        self.text.as_ref()
    }
}
