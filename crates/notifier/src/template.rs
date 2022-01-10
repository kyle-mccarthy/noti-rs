mod error;

use serde::Serialize;
use uuid::Uuid;

pub mod engine;
pub mod markup;
pub mod repository;
pub mod service;
pub mod store;

pub use engine::TemplateEngine;
pub use error::Error;
pub use markup::Markup;
pub use service::TemplateService;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TemplateId(Uuid);

impl Default for TemplateId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl TemplateId {
    /// Generate a random template id
    pub fn new() -> Self {
        Self::default()
    }
}

pub trait RegisterTemplate {
    type Template: RenderTemplate;

    fn register(&self, engine: &mut TemplateEngine) -> Result<Self::Template, Error>;
}

pub trait RenderTemplate {
    type Message;

    fn render<T: Serialize>(
        &self,
        engine: &TemplateEngine,
        data: &T,
    ) -> Result<Self::Message, Error>;
}
