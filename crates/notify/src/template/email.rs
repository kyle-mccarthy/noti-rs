use std::any::TypeId;

use lazy_static::lazy_static;
use mrml::{parse as parse_mrml, prelude::render::Options as MrmlRenderOptions};

use super::{Error, RegisteredTemplate, Template};
use crate::template::TemplateId;

lazy_static! {
    static ref DEFAULT_RENDER_OPTIONS: MrmlRenderOptions = MrmlRenderOptions::default();
}

pub trait EmailTemplate: Template {
    /// Subject to use for emails produced by this template.
    const SUBJECT: &'static str;
    /// HTML template to use for the email template. Can be MJML and handlebars.
    const HTML: &'static str;
    /// Plain text version of the email template. Can be handlebars.
    const TEXT: Option<&'static str> = None;

    fn register(manager: &mut super::TemplateManager) -> Result<(), super::Error> {
        let parsed_mrml = parse_mrml(Self::HTML).map_err(|e| Error::Parser(e.to_string()))?;

        let html_id = TemplateId::default();
        let html_template = parsed_mrml
            .render(&DEFAULT_RENDER_OPTIONS)
            .map_err(|e| Error::Parser(e.to_string()))?;

        manager
            .engine
            .register_template_string(html_id.as_str(), &html_template)?;

        let subject_id = TemplateId::default();
        manager
            .engine
            .register_template_string(subject_id.as_str(), Self::SUBJECT)?;

        let text_id = if let Some(contents) = Self::TEXT {
            let text_id = TemplateId::default();

            manager
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

        let type_id = TypeId::of::<Self>();

        let replaced_template = manager
            .templates
            .insert(type_id, RegisteredTemplate::Email(template));

        if let Some(replaced_template) = replaced_template {
            replaced_template.unregister(manager);
        }

        Ok(())
    }
}

pub struct RegisteredEmailTemplate {
    pub(super) html: TemplateId,
    pub(super) text: Option<TemplateId>,
    pub(super) subject: TemplateId,
}

pub struct RenderedEmailTemplate {
    /// The template's rendered email subject
    pub subject: String,
    /// The template's rendered html content
    pub html: String,
    /// The template's rendered plain text content
    pub text: Option<String>,
}

#[cfg(test)]
mod test_email_templates {
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use serde::Serialize;

    use super::*;
    use crate::{
        notification::EmailNotification,
        template::{RenderedTemplate, TemplateManager},
        EmailNotification,
    };

    #[derive(Serialize, EmailNotification)]
    struct Person {
        name: String,
    }

    impl EmailTemplate for Person {
        const HTML: &'static str = indoc! {"
            <mjml>
                <mj-body>
                    <mj-section>
                        <mj-column>
                            <mj-text>Hello {{ name }}!</mj-text>
                        </mj-column>
                    </mj-section>
                </mj-body>
            </mjml>
        "};
        const SUBJECT: &'static str = "Hello {{ name }}!";
        const TEXT: Option<&'static str> = Some("Hello {{ name }}!");
    }

    impl EmailNotification for Person {
        fn to(&self) -> String {
            unimplemented!()
        }
    }

    #[test]
    fn test_register_template() {
        let mut engine = TemplateManager::new();
        let result = engine.register::<Person>();
        assert!(result.is_ok());
    }

    #[test]
    fn test_render_email_template() {
        let mut engine = TemplateManager::new();
        engine
            .register::<Person>()
            .expect("Person template failed to register");

        let person = Person {
            name: "World".to_string(),
        };

        let output = engine.render(&person);

        assert!(matches!(output, Ok(RenderedTemplate::Email(_))));

        let rendered = output.unwrap().into_email().unwrap();

        assert_eq!(&rendered.subject, "Hello World!");
        assert_eq!(&rendered.text, &Some("Hello World!".to_string()));

        assert_eq!(
            &rendered.html,
        "<!doctype html><html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:v=\"urn:schemas-microsoft-com:vml\" xmlns:o=\"urn:schemas-microsoft-com:office:office\"><head><title></title><!--[if !mso]><!-- --><meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\"><!--<![endif]--><meta http-equiv=\"Content-Type\" content=\"text/html; charset=UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<style type=\"text/css\">\n#outlook a { padding: 0; }\nbody { margin: 0; padding: 0; -webkit-text-size-adjust: 100%; -ms-text-size-adjust: 100%; }\ntable, td { border-collapse: collapse; mso-table-lspace: 0pt; mso-table-rspace: 0pt; }\nimg { border: 0; height: auto; line-height: 100%; outline: none; text-decoration: none; -ms-interpolation-mode: bicubic; }\np { display: block; margin: 13px 0; }\n</style>\n<!--[if mso]>\n<noscript>\n<xml>\n<o:OfficeDocumentSettings>\n  <o:AllowPNG/>\n  <o:PixelsPerInch>96</o:PixelsPerInch>\n</o:OfficeDocumentSettings>\n</xml>\n</noscript>\n<![endif]-->\n<!--[if lte mso 11]>\n<style type=\"text/css\">\n.mj-outlook-group-fix { width:100% !important; }\n</style>\n<![endif]-->\n<!--[if !mso]><!--><link href=\"https://fonts.googleapis.com/css?family=Ubuntu:300,400,500,700\" rel=\"stylesheet\" type=\"text/css\"><style type=\"text/css\">@import url(https://fonts.googleapis.com/css?family=Ubuntu:300,400,500,700);</style><!--<![endif]--><style type=\"text/css\">@media only screen and (min-width:480px) { .mj-column-per-100 { width:100% !important; max-width:100%; }  }</style><style media=\"screen and (min-width:480px)\">.moz-text-html .mj-column-per-100 { width:100% !important; max-width:100%; } </style></head><body style=\"word-spacing:normal;\"><div><!--[if mso | IE]><table border=\"0\" cellpadding=\"0\" cellspacing=\"0\" align=\"center\" width=\"600\" style=\"width:600px;\"><tr><td style=\"line-height:0px;font-size:0px;mso-line-height-rule:exactly;\"><![endif]--><div style=\"margin:0px auto;max-width:600px;\"><table border=\"0\" cellpadding=\"0\" cellspacing=\"0\" role=\"presentation\" align=\"center\" style=\"width:100%;\"><tbody><tr><td style=\"direction:ltr;font-size:0px;padding:20px 0;text-align:center;\"><!--[if mso | IE]><table border=\"0\" cellpadding=\"0\" cellspacing=\"0\" role=\"presentation\"><tr><td style=\"vertical-align:top;width:600px;\"><![endif]--><div class=\"mj-outlook-group-fix mj-column-per-100\" style=\"font-size:0px;text-align:left;direction:ltr;display:inline-block;vertical-align:top;width:100%;\"><table border=\"0\" cellpadding=\"0\" cellspacing=\"0\" role=\"presentation\" width=\"100%\" style=\"vertical-align:top;\"><tbody><tr><td align=\"left\" style=\"font-size:0px;padding:10px 25px;word-break:break-word;\"><div style=\"font-family:Ubuntu, Helvetica, Arial, sans-serif;font-size:13px;line-height:1;text-align:left;color:#000000;\">Hello World!</div></td></tr></tbody></table></div><!--[if mso | IE]></td></tr></table><![endif]--></td></tr></tbody></table></div><!--[if mso | IE]></td></tr></table><![endif]--></div></body></html>");
    }
}
