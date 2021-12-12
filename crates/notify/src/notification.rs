use std::any::Any;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

mod manager;
pub use manager::Manager;

use crate::id::Id;

pub trait Notification: Any + Serialize + DeserializeOwned {
    type Id: Id;

    fn id() -> Self::Id;
}
