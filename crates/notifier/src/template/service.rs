use super::{
    engine::TemplateEngine, store::TemplateStore, Error, RegisterTemplate, RenderTemplate,
};
use crate::{Id, Notification};

#[derive(Default)]
pub struct TemplateService<I: Id, T: RenderTemplate> {
    engine: TemplateEngine,
    store: TemplateStore<I, T>,
}
impl<I: Id, T: RenderTemplate> TemplateService<I, T> {
    pub fn new() -> Self {
        Self {
            engine: TemplateEngine::new(),
            store: TemplateStore::new(),
        }
    }

    pub fn register<N: Notification<Id = I>, S: RegisterTemplate<Template = T>>(
        &mut self,
        source: S,
    ) -> Result<(), Error> {
        let template = source.register(&mut self.engine)?;
        self.store.insert(N::id(), template);
        Ok(())
    }

    pub fn render<N: Notification<Id = I>>(&self, notification: &N) -> Result<T::Message, Error> {
        let template = self
            .store
            .get(&N::id())
            .ok_or_else(|| Error::UnknownNotification(N::id().to_string()))?;
        template.render(&self.engine, notification)
    }
}
