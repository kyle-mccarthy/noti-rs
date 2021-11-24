use std::any::Any;

use crate::template::Template;

pub trait Notification: Any {
    type Template: Template;
}
