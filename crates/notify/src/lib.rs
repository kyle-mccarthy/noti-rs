use template::Template;

pub mod channel;
pub mod message;
pub mod notification;
pub mod template;

pub struct Notify;

impl Notify {
    pub fn register_template<T: Template>(&mut self, template: T) {
        todo!()
    }
}
