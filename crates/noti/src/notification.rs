use std::any::Any;

use serde::{de::DeserializeOwned, Serialize};

mod store;

pub(crate) use store::{Composite, Store};

use crate::id::Id;

pub trait Notification: Any + Serialize + DeserializeOwned {
    type Id: Id;

    fn id() -> Self::Id;
}
