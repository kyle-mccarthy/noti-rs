use std::collections::{hash_map::Entry, HashMap};

use crate::{id::Id, template::Composite as Templates};

#[derive(Default)]
pub struct Store<T: Id> {
    notifications: HashMap<T, Composite>,
}

impl<T: Id> Store<T> {
    pub fn entry(&mut self, id: T) -> Entry<T, Composite> {
        self.notifications.entry(id)
    }

    pub fn get(&self, id: T) -> Option<&Composite> {
        self.notifications.get(&id)
    }
}

#[derive(Default)]
pub struct Composite {
    pub(crate) templates: Templates,
}
