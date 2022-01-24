use uuid::Uuid;

pub mod engine;
pub mod error;
pub mod markup;
pub mod registry;
pub mod service;

pub use engine::TemplateEngine;
pub use error::Error as TemplateError;
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
