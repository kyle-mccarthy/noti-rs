use std::collections::HashMap;

use liquid::{Object, Template};
use serde::Serialize;

use super::{Error, TemplateId};

pub struct RenderContext(liquid::Object);

impl RenderContext {
    /// Create a new render context wrapping the [`Object`]
    pub fn new(data: Object) -> Self {
        Self(data)
    }

    /// Create the rendering context from the data
    pub fn with_data<T: Serialize>(data: &T) -> Result<Self, Error> {
        Ok(Self(liquid::to_object(data).map_err(Error::InvalidData)?))
    }
}

#[derive(Default)]
pub struct TemplateEngine {
    templates: HashMap<TemplateId, Template>,
}

impl TemplateEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_context<T: Serialize>(&self, data: &T) -> Result<RenderContext, Error> {
        RenderContext::with_data(data)
    }

    /// Register the string as a template
    pub fn register_template(&mut self, template: &str) -> Result<TemplateId, Error> {
        let parser = liquid::ParserBuilder::with_stdlib()
            .build()
            .map_err(Error::Parse)?;

        let template = parser.parse(template).map_err(Error::Parse)?;

        let id = TemplateId::new();

        self.templates.insert(id, template);

        Ok(id)
    }

    /// Render the template to a string using the context's data
    pub fn render(&self, id: TemplateId, ctx: &RenderContext) -> Result<String, Error> {
        let template = self.templates.get(&id).ok_or(Error::UnknownTemplate(id))?;

        template.render(&ctx.0).map_err(Error::Render)
    }
}

#[cfg(test)]
mod test_template_engine {
    use super::*;

    #[test]
    fn test_register_and_render() {
        let mut engine = TemplateEngine::new();
        let id = engine.register_template("Hello, {{ name }}!").unwrap();

        let ctx = RenderContext::new(liquid::object!({
            "name": "World"
        }));

        let output = engine.render(id, &ctx).unwrap();

        assert_eq!(&output, "Hello, World!");
    }
}
