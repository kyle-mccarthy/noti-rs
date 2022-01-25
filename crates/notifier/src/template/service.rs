use std::any::{Any, TypeId};

use super::{
    engine::{RenderContext, TemplateEngine},
    registry::TemplateRegistry,
    TemplateError, TemplateId,
};
use crate::{channel::ChannelType, Error, Id};

#[derive(Default)]
pub struct TemplateService<I: Id> {
    engine: TemplateEngine,
    registry: TemplateRegistry<I>,
}
impl<I: Id> TemplateService<I> {
    pub fn new() -> Self {
        Self {
            engine: TemplateEngine::new(),
            registry: TemplateRegistry::new(),
        }
    }

    /// Register the template with the service for the given channel and
    /// notification. The template is `Box<dyn Any>`, the channel will
    /// downcast this to get the concrete type.
    pub fn register_template(
        &mut self,
        notification_id: I,
        channel_type: ChannelType,
        template: Box<dyn Any>,
    ) {
        self.registry
            .register(notification_id, channel_type, template)
    }

    /// Get a reference to the template registered for the channel and
    /// notification.
    pub fn get_template<T: Any>(
        &self,
        notification_id: I,
        channel_type: ChannelType,
    ) -> Result<&T, Error> {
        let template = self
            .registry
            .get_template(notification_id, channel_type)
            .ok_or_else(|| TemplateError::NotFound {
                channel_type,
                notification_id: notification_id.to_string(),
            })?;

        let template = template.downcast_ref::<T>().ok_or(Error::Downcast {
            context: Some("Failed to downcast the template into T"),
            found: (&*template).type_id(),
            expected: TypeId::of::<T>(),
        })?;

        Ok(template)
    }

    pub fn engine(&self) -> &TemplateEngine {
        &self.engine
    }

    pub fn engine_mut(&mut self) -> &mut TemplateEngine {
        &mut self.engine
    }

    /// Render the template with the data from the context.
    pub fn render_template(
        &self,
        template_id: TemplateId,
        context: &RenderContext,
    ) -> Result<String, TemplateError> {
        self.engine().render(template_id, context)
    }
}
