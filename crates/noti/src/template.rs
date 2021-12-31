use serde::Serialize;

use crate::channel::ChannelType;

mod composite;
mod engine;
mod id;
mod markup;

pub use composite::Composite;
pub use engine::Engine;
pub use id::TemplateId;
pub use markup::Markup;

use self::markup::MarkupType;

pub trait Register {
    type Output: RenderTemplate;

    fn channel(&self) -> ChannelType;
    fn register(&self, engine: &mut Engine) -> Result<Self::Output, Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Template contained invalid markup")]
    Markup {
        source: anyhow::Error,
        markup_type: MarkupType,
        context: Option<&'static str>,
    },

    #[error("Error while parsing the template: {0:?}")]
    Parse(#[from] handlebars::TemplateError),

    #[error("Failed to render the template: {0:?}")]
    Render(#[from] handlebars::RenderError),
}

pub trait RenderTemplate {
    type MessageBuilder;

    fn render<T: Serialize>(
        &self,
        engine: &Engine,
        data: &T,
    ) -> Result<Self::MessageBuilder, Error>;
}
