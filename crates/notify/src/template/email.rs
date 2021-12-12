use serde::Serialize;

use super::{
    store::TemplateStore, Error, RegisteredTemplate, Render, RenderedTemplate, Template, TemplateId,
};
use crate::{channel::ChannelType, message::email::EmailContents};

pub struct EmailTemplate<'a> {
    /// The template for the email's subject.
    pub subject: &'a str,
    /// The template for the email's HTML content. Preprased as mjml.
    pub html: &'a str,
    /// The optional template for the email's plain text content.
    pub text: Option<&'a str>,
}

impl<'a> super::sealed::Sealed for EmailTemplate<'a> {}

impl<'a> Template for EmailTemplate<'a> {
    fn channel(&self) -> ChannelType {
        ChannelType::Email
    }

    fn register(&self, store: &mut TemplateStore) -> Result<RegisteredTemplate, Error> {
        let subject = store.register(self.subject)?;

        let html = mrml::parse(self.html)
            .map_err(|e| Error::PreParse(anyhow::Error::msg(e.to_string())))?
            .to_string();
        let html = store.register(&html)?;

        let text = if let Some(text) = &self.text {
            Some(store.register(text)?)
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
        store: &TemplateStore,
        data: &T,
    ) -> Result<super::RenderedTemplate, Error> {
        let subject = store.render(&self.subject, data)?;
        let html = store.render(&self.html, data)?;
        let text = if let Some(text) = &self.text {
            Some(store.render(text, data)?)
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
