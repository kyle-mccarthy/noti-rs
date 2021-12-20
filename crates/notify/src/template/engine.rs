use handlebars::Handlebars;
use serde::Serialize;

use super::{Error, TemplateId};

#[derive(Default)]
pub struct Engine<'a>(Handlebars<'a>);

impl<'a> Engine<'a> {
    pub fn register(&mut self, template: &str) -> Result<TemplateId, Error> {
        let id = TemplateId::new();

        self.0
            .register_template_string(id.as_str(), template)
            .map_err(Error::Parse)?;

        Ok(id)
    }

    pub fn register_partial(&mut self, name: &str, partial: &str) -> Result<(), Error> {
        self.0
            .register_partial(name, partial)
            .map_err(Error::Parse)
    }

    pub fn render(&self, id: &TemplateId, data: &impl Serialize) -> Result<String, Error> {
        self.0
            .render(id.as_str(), data)
            .map_err(Error::Render)
    }

    pub fn render_partial(&self, name: &str, data: &impl Serialize) -> Result<String, Error> {
        self.0.render(name, data).map_err(Error::Render)
    }
}
