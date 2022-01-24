use std::any::Any;

use serde::{Deserialize, Serialize};

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
    + 'static
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
        + 'static
{
}

pub trait Notification: Sized + Any + Serialize {
    type Id: Id;

    fn id() -> Self::Id
    where
        Self: Sized;
}
