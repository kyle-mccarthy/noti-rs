use std::{hash::Hash, marker::PhantomData};

use uuid::Uuid;

pub trait Id: Clone + Copy + PartialEq + PartialOrd + Hash + Eq {}

impl Id for u64 {}
impl Id for i64 {}
impl Id for Uuid {}
impl<'a> Id for &'a str {}

pub trait ContactRepository<T: Id> {
    // fn find_by_id(&self, id: &T) -> Option<Contact>;
}

pub trait PreferenceRepository<C: Id, N: Id, T: Id> {
    fn category_enabled(&self, contact_id: &C, category_id: &T) -> bool;
    fn notification_enabled(&self, contact_id: &C, notification_id: &N) -> bool;
}

pub struct Driver<C: Id, N: Id, T: Id, CR: ContactRepository<C>, PR: PreferenceRepository<C, N, T>>
{
    pub contacts: CR,
    pub preferences: PR,
    _ph: PhantomData<(C, N, T)>,
}

// pub struct Notify<C: ContactRepository, P: PreferenceRepository> {
//     contacts: C,
//     preferences: P,
//     templates: TemplateManager,
// }
