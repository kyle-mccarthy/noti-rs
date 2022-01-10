use super::{engine::RenderContext, Error, TemplateId};

pub trait TemplateRepository {
    /// Register a template with the repository.
    fn register(&mut self, template: &str) -> Result<TemplateId, Error>;

    /// Render a template to a string using the render context
    fn render(&self, ctx: &RenderContext, id: &TemplateId) -> Result<String, Error>;
}
