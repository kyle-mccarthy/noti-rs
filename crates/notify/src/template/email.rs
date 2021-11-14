use std::any::{Any, TypeId};

use lazy_static::lazy_static;
use mrml::parse;
use mrml::prelude::render::Options as MrmlRenderOptions;
use serde::Serialize;

use crate::template::TemplateId;

use super::{Error, Register, RegisteredTemplate, Template, TemplateEngine};

lazy_static! {
    static ref DEFAULT_RENDER_OPTIONS: MrmlRenderOptions = MrmlRenderOptions::default();
}

pub trait EmailTemplate: Any + Serialize {
    /// Subject to use for emails produced by this template.
    const SUBJECT: &'static str;
    /// HTML template to use for the email template. Can be MJML and handlebars.
    const HTML: &'static str;
    /// Plain text version of the email template. Can be handlebars.
    const TEXT: Option<&'static str> = None;
}


pub(super) struct RegisteredEmailTemplate {
    pub html: TemplateId,
    pub text: Option<TemplateId>,
    pub subject: TemplateId,
}

pub struct RenderedEmailTemplate {
    pub subject: String,
    pub html: String,
    pub text: Option<String>,
}

impl<T: EmailTemplate> Template for T {
    fn register(engine: &mut super::TemplateEngine) -> Result<(), super::Error> {
        let parsed_mrml = parse(T::HTML).map_err(|e| Error::Parser(e.to_string()))?;

        let html_id = TemplateId::default();
        let html_template = parsed_mrml
            .render(&DEFAULT_RENDER_OPTIONS)
            .map_err(|e| Error::Parser(e.to_string()))?;

        engine
            .engine
            .register_template_string(html_id.as_str(), &html_template)?;

        let subject_id = TemplateId::default();
        engine
            .engine
            .register_template_string(subject_id.as_str(), Self::SUBJECT)?;

        let text_id = if let Some(contents) = Self::TEXT {
            let text_id = TemplateId::default();

            engine
                .engine
                .register_template_string(text_id.as_str(), contents)?;

            Some(text_id)
        } else {
            None
        };

        let template = RegisteredEmailTemplate {
            html: html_id,
            subject: subject_id,
            text: text_id,
        };

        let type_id = TypeId::of::<T>();

        // TODO: cleanup/remove the templates in the HBS engine if insert returns Some(Template)
        engine
            .templates
            .insert(type_id, RegisteredTemplate::Email(template));

        Ok(())
    }
}

mod sealed {
    use serde::Serialize;

    pub trait Sealed: std::any::Any + Serialize {}
}

#[cfg(test)]
mod test_email_templates {
    use crate::template::TemplateEngine;

    use super::*;
    use indoc::indoc;
    use serde::Serialize;

    #[derive(Serialize)]
    struct Person {
        name: String,
    }

    impl EmailTemplate for Person {
        const HTML: &'static str = indoc! {"
            <mrml>
                <mj-body>
                    <mj-section>
                        <mj-column>
                            <mj-text>Hello {{ name }}!</mj-text>
                        </mj-column>
                    </mj-section>
                </mj-body>
            </mrml>
        "};

        const SUBJECT: &'static str = "Hello {{ name }}!";

        const TEXT: Option<&'static str> = Some("Hello {{ name }}!");
    }

    #[test]
    fn test_register_template() {
        let mut engine = TemplateEngine::new();
    }
}
