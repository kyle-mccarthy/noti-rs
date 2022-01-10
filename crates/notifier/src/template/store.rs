use std::collections::HashMap;

use super::RenderTemplate;
use crate::Id;

#[derive(Default)]
pub struct TemplateStore<I: Id, T: RenderTemplate>(HashMap<I, T>);

impl<I: Id, T: RenderTemplate> TemplateStore<I, T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, key: I, value: T) -> Option<T> {
        self.0.insert(key, value)
    }

    pub fn get(&self, key: &I) -> Option<&T> {
        self.0.get(key)
    }
}
