use crate::{
    channel::ChannelType,
    message::{email::Email, Message},
    template::{RenderedEmailTemplate, RenderedTemplate, Template},
};

pub trait Notification: Template {
    const CHANNEL_TYPE: &'static ChannelType;

    fn into_message(self, template: RenderedTemplate) -> Message;
}

pub trait EmailNotification {
    /// The recipient/who to send the email to
    fn to(&self) -> String;

    /// The sender/who send the email
    fn from(&self) -> Option<String> {
        None
    }

    fn build(&self, rendered: RenderedEmailTemplate) -> Email {
        Email {
            to: self.to(),
            from: self.from(),
            subject: rendered.subject,
            html: rendered.html,
            text: rendered.text,
        }
    }
}
