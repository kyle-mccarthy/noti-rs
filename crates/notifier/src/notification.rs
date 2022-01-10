use std::any::Any;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait Id:
    std::fmt::Debug
    + std::fmt::Display
    + Clone
    + Copy
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + std::hash::Hash
    + Serialize
    + Deserialize<'static>
    + Sync
    + Send
{
}

impl<T> Id for T where
    T: std::fmt::Debug
        + std::fmt::Display
        + Clone
        + Copy
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + std::hash::Hash
        + Serialize
        + Deserialize<'static>
        + Sync
        + Send
{
}

pub trait Notification: Any + Serialize + DeserializeOwned {
    type Id: Id;

    fn id() -> Self::Id;
}
