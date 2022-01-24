use std::{
    any::TypeId,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{template::TemplateId, Channel, Error, Id, Notification, TemplateError};

#[derive(Serialize, Deserialize, Debug)]
pub struct TestContact(pub String);

#[derive(Serialize, Deserialize, Debug)]
pub struct TestMessageContents {
    pub output: String,
    pub notification_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestMessage {
    pub contact: TestContact,
    pub contents: TestMessageContents,
}

#[derive(Default)]
pub struct TestChannel {
    pub messages: Arc<Mutex<Vec<TestMessage>>>,
}

impl Clone for TestChannel {
    fn clone(&self) -> Self {
        Self {
            messages: self.messages.clone(),
        }
    }
}

pub struct TestTemplate(pub &'static str);
pub struct TestRegisteredTemplate(TemplateId);

#[async_trait]
impl<I: Id> Channel<I> for TestChannel {
    type Contact = TestContact;
    type Message = TestMessage;
    type RenderedTemplate = TestMessageContents;
    type UserTemplate = TestTemplate;

    /// Create a message that has the contact as the recipient
    fn create_message(
        &self,
        contact: TestContact,
        contents: TestMessageContents,
    ) -> Result<Self::Message, Error> {
        let message = TestMessage { contact, contents };
        Ok(message)
    }

    async fn send(&self, message: Self::Message) -> Result<(), crate::Error> {
        self.messages.lock().unwrap().push(message);
        Ok(())
    }

    fn register_template(
        &self,
        notification_id: I,
        source: Self::UserTemplate,
        template_service: &mut crate::template::TemplateService<I>,
    ) -> Result<(), Error> {
        let template_id = template_service.engine_mut().register(source.0)?;
        let template = TestRegisteredTemplate(template_id);

        let channel_type = <Self as Channel<I>>::channel_type(self);

        template_service.register_template(notification_id, channel_type, Box::new(template));

        Ok(())
    }

    fn render_template(
        &self,
        notification_id: I,
        context: &crate::template::engine::RenderContext,
        template_service: &crate::template::TemplateService<I>,
    ) -> Result<Self::RenderedTemplate, Error> {
        let channel_type = <Self as Channel<I>>::channel_type(self);

        let template = template_service
            .get_template(notification_id, channel_type)
            .ok_or_else(|| TemplateError::NotFound {
                channel_type,
                notification_id: notification_id.to_string(),
            })?;

        let template =
            template
                .downcast_ref::<TestRegisteredTemplate>()
                .ok_or(Error::Downcast {
                    context: Some("Failed to downcast the template into TestRegisteredTemplate"),
                    found: (&*template).type_id(),
                    expected: TypeId::of::<TestRegisteredTemplate>(),
                })?;

        let output = template_service.render_template(template.0, context)?;

        let contents = TestMessageContents {
            output,
            notification_id: notification_id.to_string(),
        };

        Ok(contents)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestNotification {
    pub id: usize,
    pub message: String,
}

impl TestNotification {
    pub fn new(id: usize, message: String) -> Self {
        Self { id, message }
    }
}

impl Notification for TestNotification {
    type Id = &'static str;

    fn id() -> Self::Id
    where
        Self: Sized,
    {
        "test_notification"
    }
}
