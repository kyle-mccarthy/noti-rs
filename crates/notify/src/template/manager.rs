use serde::Serialize;

use super::{Error, TemplateId};

#[derive(Default)]
pub struct Manager<'a> {
    engine: handlebars::Handlebars<'a>,
}

impl<'a> Manager<'a> {
    pub(crate) fn register(&mut self, template: &str) -> Result<TemplateId, Error> {
        let id = TemplateId::new();

        self.engine.register_template_string(&id.0, template)?;

        Ok(id)
    }

    /// removes a template from the engine
    pub fn unregister(&mut self, id: TemplateId) {
        self.engine.unregister_template(&id.0)
    }

    /// Attempts to render the contents of the template using the provided data.
    pub fn render<T: Serialize>(&self, template: &TemplateId, data: &T) -> Result<String, Error> {
        let output = self.engine.render(&template.0, data)?;
        Ok(output)
    }
}
