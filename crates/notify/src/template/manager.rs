use serde::Serialize;

use super::{Error, TemplateId};

#[derive(Default)]
pub struct Manager<'a> {
    engine: handlebars::Handlebars<'a>,
    // templates: HashMap<TypeId, RegisteredTemplate>,
}

impl<'a> Manager<'a> {
    pub(crate) fn register(&mut self, template: &str) -> Result<TemplateId, Error> {
        let id = TemplateId::new();

        self.engine.register_template_string(&id.0, template)?;

        Ok(id)
    }

    pub fn render<T: Serialize>(&self, template: &TemplateId, data: &T) -> Result<String, Error> {
        let output = self.engine.render(&template.0, data)?;
        Ok(output)
    }
}
