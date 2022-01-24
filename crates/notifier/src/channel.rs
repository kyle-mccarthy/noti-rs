use std::any::{Any, TypeId};

use async_trait::async_trait;

pub mod registry;

use crate::{
    contact::{Contact, DynContact},
    message::{DynMessage, DynMessageContents, Message},
    template::{engine::RenderContext, TemplateService},
    Error, Id,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChannelType(TypeId);

impl ChannelType {
    pub fn of<M: Message, C: Contact>() -> Self {
        Self(TypeId::of::<(M, C)>())
    }
}

#[async_trait]
pub trait Channel<I: Id>: Any + Sync + Send {
    /// The recipient of a message.
    type Contact: Contact;

    /// The message sent for a notification.
    type Message: Message;

    /// The template that end user's are expected to user for this channel.
    type UserTemplate: Any + Send;

    /// The result of rendering a registered template for this channel. This is
    /// combined with a contact for this channel should be sufficient for
    /// creating a message.
    type RenderedTemplate: Any + Send;

    /// Unique ID for the channel.
    fn channel_type(&self) -> ChannelType {
        ChannelType::of::<Self::Message, Self::Contact>()
    }

    /// Create a message that has the contact as the recipient
    fn create_message(
        &self,
        contact: Self::Contact,
        contents: Self::RenderedTemplate,
    ) -> Result<Self::Message, Error>;

    /// Send a message using the channel's provider
    async fn send(&self, message: Self::Message) -> Result<(), Error>;

    /// Register a user's template with the template service.
    fn register_template(
        &self,
        notification_id: I,
        source: Self::UserTemplate,
        template_service: &mut TemplateService<I>,
    ) -> Result<(), Error>;

    /// Render the template using the template service.
    fn render_template(
        &self,
        notification_id: I,
        context: &RenderContext,
        template_service: &TemplateService<I>,
    ) -> Result<Self::RenderedTemplate, Error>;

    /// Convert the channel into a DynChannel
    fn into_dyn(self) -> Box<dyn DynChannel<I>>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

#[async_trait]
pub trait DynChannel<I: Id>: Any {
    async fn send_dyn_message(&self, message: DynMessage) -> Result<(), Error>;

    fn create_dyn_message(
        &self,
        contact: DynContact,
        contents: DynMessageContents,
    ) -> Result<DynMessage, Error>;

    fn register_dyn_template(
        &self,
        notification_id: I,
        template: Box<dyn Any>,
        template_service: &mut TemplateService<I>,
    ) -> Result<(), Error>;

    fn render_dyn_template(
        &self,
        notification_id: I,
        context: &RenderContext,
        template_service: &TemplateService<I>,
    ) -> Result<DynMessageContents, Error>;

    fn get_channel_type(&self) -> ChannelType;
}

#[async_trait]
impl<I: Id, T: Channel<I>> DynChannel<I> for T {
    fn get_channel_type(&self) -> ChannelType {
        <Self as Channel<I>>::channel_type(self)
    }

    async fn send_dyn_message(&self, message: DynMessage) -> Result<(), Error> {
        let message = message
            .take_message()
            .downcast::<T::Message>()
            .map_err(|inner| Error::Downcast {
                context: Some("DynChannel to downcast contact to EmailAddress"),
                found: (&*inner).type_id(),
                expected: TypeId::of::<T::Message>(),
            })?;

        <Self as Channel<I>>::send(self, *message).await?;

        Ok(())
    }

    fn create_dyn_message(
        &self,
        contact: DynContact,
        contents: DynMessageContents,
    ) -> Result<DynMessage, Error> {
        let contact = contact
            .take_contact()
            .downcast::<T::Contact>()
            .map_err(|inner| Error::Downcast {
                context: Some("DynChannel could not be downcasted to Self::Channel"),
                found: (&*inner).type_id(),
                expected: TypeId::of::<T::Contact>(),
            })?;

        let contents = contents
            .take_contents()
            .downcast::<T::RenderedTemplate>()
            .map_err(|inner| Error::Downcast {
                context: Some("DynContents could not be downcasted to Self::Template::Output"),
                found: (&*inner).type_id(),
                expected: TypeId::of::<T::RenderedTemplate>(),
            })?;

        let message = <Self as Channel<I>>::create_message(self, *contact, *contents)?;

        let dyn_message = DynMessage::new(message, self.channel_type());

        Ok(dyn_message)
    }

    fn register_dyn_template(
        &self,
        notification_id: I,
        template: Box<dyn Any>,
        template_service: &mut TemplateService<I>,
    ) -> Result<(), Error> {
        let source = template
            .downcast::<T::UserTemplate>()
            .map_err(|inner| Error::Downcast {
                context: Some("DynContents could not be downcasted to Self::Template::Output"),
                found: (&*inner).type_id(),
                expected: TypeId::of::<T::UserTemplate>(),
            })?;

        <Self as Channel<I>>::register_template(self, notification_id, *source, template_service)?;

        Ok(())
    }

    fn render_dyn_template(
        &self,
        notification_id: I,
        context: &RenderContext,
        template_service: &TemplateService<I>,
    ) -> Result<DynMessageContents, Error> {
        let output = <Self as Channel<I>>::render_template(
            self,
            notification_id,
            context,
            template_service,
        )?;

        Ok(DynMessageContents::new(output, self.channel_type()))
    }
}

static_assertions::assert_obj_safe!(DynChannel<&'static str>);
