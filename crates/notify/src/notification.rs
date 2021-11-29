use std::any::Any;

use serde::Serialize;

mod manager;
pub use manager::Manager;

pub trait Notification: Any + Serialize {
    type Id;

    fn id() -> Self::Id;
}
