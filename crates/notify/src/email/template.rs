use serde::Serialize;

use super::EmailBuilder;
use crate::{
    id::Id,
    template::{Engine, Markup, Renderable, TemplateId},
    RegisterTemplate,
};

pub struct EmailTemplate<'a> {
    /// The template for the email's subject.
    pub subject: &'a str,
    /// The template for the email's HTML content. Preprased as mjml.
    pub html: Markup<'a>,
    /// The optional template for the email's plain text content.
    pub text: Option<&'a str>,
}

impl<'a> RegisterTemplate for EmailTemplate<'a> {
    fn register<N: Id>(
        self,
        notification_id: N,
        instance: &mut crate::Notify<N>,
    ) -> Result<(), crate::Error> {
        let html = self.html.parse()?;

        let html = instance.templates.register(&html)?;
        let subject = instance.templates.register(self.subject)?;

        let text = match self.text {
            Some(text) => Some(instance.templates.register(text)?),
            _ => None,
        };

        let template = RegisteredEmailTemplate {
            subject,
            text,
            html,
        };

        instance
            .notifications
            .entry(notification_id)
            .or_default()
            .templates
            .set_email(Some(template));

        Ok(())
    }
}

pub struct RegisteredEmailTemplate {
    subject: TemplateId,
    html: TemplateId,
    text: Option<TemplateId>,
}

impl Renderable for RegisteredEmailTemplate {
    type MessageBuilder = EmailBuilder;

    fn render<T: Serialize>(
        &self,
        engine: &Engine,
        data: &T,
    ) -> Result<Self::MessageBuilder, crate::template::Error> {
        let html = engine.render(&self.html, data)?;
        let subject = engine.render(&self.subject, data)?;

        let mut builder = EmailBuilder::new().html(html).subject(subject);

        if let Some(text) = &self.text {
            let text = engine.render(text, data)?;
            builder = builder.text(text);
        }

        Ok(builder)
    }
}
