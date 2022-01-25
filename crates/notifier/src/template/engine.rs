use std::collections::HashMap;

use liquid::{Object, Template};
use serde::Serialize;

use super::{TemplateError, TemplateId};

pub struct RenderContext(liquid::Object);

impl RenderContext {
    /// Create a new render context wrapping the [`Object`]
    pub fn new(data: Object) -> Self {
        Self(data)
    }

    /// Create the rendering context from the data
    pub fn with_data<T: Serialize>(data: &T) -> Result<Self, TemplateError> {
        Ok(Self(
            liquid::to_object(data).map_err(TemplateError::InvalidData)?,
        ))
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

    pub fn create_context<T: Serialize>(&self, data: &T) -> Result<RenderContext, TemplateError> {
        RenderContext::with_data(data)
    }

    /// Register the string as a template
    pub fn register(&mut self, template: &str) -> Result<TemplateId, TemplateError> {
        let parser = liquid::ParserBuilder::with_stdlib()
            .build()
            .map_err(TemplateError::Parse)?;

        let template = parser.parse(template).map_err(TemplateError::Parse)?;

        let id = TemplateId::new();

        self.templates.insert(id, template);

        Ok(id)
    }

    /// Render the template to a string using the context's data
    pub fn render(&self, id: TemplateId, ctx: &RenderContext) -> Result<String, TemplateError> {
        let template = self
            .templates
            .get(&id)
            .ok_or(TemplateError::UnknownTemplate(id))?;

        template.render(&ctx.0).map_err(TemplateError::Render)
    }
}

#[cfg(test)]
mod test_template_engine {
    use super::*;

    #[test]
    fn test_register_and_render() {
        let mut engine = TemplateEngine::new();
        let id = engine.register("Hello, {{ name }}!").unwrap();

        let ctx = RenderContext::new(liquid::object!({
            "name": "World"
        }));

        let output = engine.render(id, &ctx).unwrap();

        assert_eq!(&output, "Hello, World!");
    }
}
