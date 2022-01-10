use notifier::template::{
    Error as TemplateError, Markup, RegisterTemplate, RenderTemplate, TemplateEngine, TemplateId,
};

use crate::EmailContents;

pub struct EmailTemplate<'a> {
    /// The template for the email's subject.
    pub subject: &'a str,
    /// The template for the email's HTML content. Preprased as mjml.
    pub html: Markup<'a>,
    /// The optional template for the email's plain text content.
    pub text: Option<&'a str>,
}

impl<'a> RegisterTemplate for EmailTemplate<'a> {
    type Template = RegisteredEmailTemplate;

    fn register(&self, engine: &mut TemplateEngine) -> Result<Self::Template, TemplateError> {
        let html = engine.register_template(&self.html.parse()?)?;
        let subject = engine.register_template(self.subject)?;

        let text = if let Some(text) = self.text {
            Some(engine.register_template(text)?)
        } else {
            None
        };

        Ok(RegisteredEmailTemplate {
            html,
            subject,
            text,
        })
    }
}

pub struct RegisteredEmailTemplate {
    subject: TemplateId,
    html: TemplateId,
    text: Option<TemplateId>,
}

impl RenderTemplate for RegisteredEmailTemplate {
    type Message = EmailContents;

    fn render<T: serde::Serialize>(
        &self,
        engine: &TemplateEngine,
        data: &T,
    ) -> Result<Self::Message, TemplateError> {
        let ctx = engine.create_context(data)?;

        let html = engine.render(self.html, &ctx)?;
        let subject = engine.render(self.subject, &ctx)?;

        let text = if let Some(text) = self.text {
            Some(engine.render(text, &ctx)?)
        } else {
            None
        };

        Ok(EmailContents::new(subject, html, text))
    }
}
