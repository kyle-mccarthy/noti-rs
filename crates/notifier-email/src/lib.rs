use async_trait::async_trait;
use notifier::{template::TemplateService, Channel, Error, Id, Provider};

pub mod contact;
pub mod message;
pub mod provider;
pub mod template;

pub use contact::EmailAddress;
pub use message::{EmailContents, EmailMessage};
use provider::EmailProvider;
pub use template::EmailTemplate;
use template::RegisteredEmailTemplate;

pub struct Options {
    /// The default sender to use for the email
    default_sender: EmailAddress,
    /// An optional reply_to email address
    reply_to: Option<EmailAddress>,
}

impl Options {
    pub fn new(default_sender: EmailAddress, reply_to: Option<EmailAddress>) -> Self {
        Self {
            default_sender,
            reply_to,
        }
    }
}

pub struct EmailChannel {
    provider: EmailProvider,
    options: Options,
}

impl EmailChannel {
    pub fn new(provider: impl Provider<Message = EmailMessage>, options: Options) -> Self {
        Self {
            provider: EmailProvider::new(provider),
            options,
        }
    }

    pub async fn send(&self, message: EmailMessage) -> Result<(), Error> {
        self.provider.send(message).await?;
        Ok(())
    }
}

#[async_trait]
impl<I: Id + 'static> Channel<I> for EmailChannel {
    type Contact = EmailAddress;
    type Message = EmailMessage;
    type RenderedTemplate = EmailContents;
    type UserTemplate = EmailTemplate<'static>;

    /// Create a message that has the contact as the recipient
    fn create_message(
        &self,
        contact: Self::Contact,
        contents: EmailContents,
    ) -> Result<Self::Message, Error> {
        let mut message = EmailMessage::new(contact, self.options.default_sender.clone(), contents);

        message.set_reply_to(self.options.reply_to.clone());

        Ok(message)
    }

    /// Send a message using the channel's provider
    async fn send(&self, message: Self::Message) -> Result<(), Error> {
        self.send(message).await
    }

    fn register_template(
        &self,
        notification_id: I,
        source: Self::UserTemplate,
        template_service: &mut TemplateService<I>,
    ) -> Result<(), Error> {
        let html = source.html.parse()?;

        let html_template_id = template_service.engine_mut().register(&html)?;
        let subject_template_id = template_service.engine_mut().register(source.subject)?;

        let text_tempalte_id = if let Some(text) = source.text {
            Some(template_service.engine_mut().register(text)?)
        } else {
            None
        };

        let template = RegisteredEmailTemplate {
            html: html_template_id,
            subject: subject_template_id,
            text: text_tempalte_id,
        };

        let channel_type = <Self as Channel<I>>::channel_type(self);

        template_service.register_template(notification_id, channel_type, Box::new(template));

        Ok(())
    }

    fn render_template(
        &self,
        notification_id: I,
        context: &notifier::template::engine::RenderContext,
        template_service: &TemplateService<I>,
    ) -> Result<Self::RenderedTemplate, Error> {
        let channel_type = <Self as Channel<I>>::channel_type(self);

        let template = template_service
            .get_template::<RegisteredEmailTemplate>(notification_id, channel_type)?;

        let html = template_service.render_template(template.html, context)?;

        let subject = template_service.render_template(template.subject, context)?;

        let text = if let Some(template_id) = template.text {
            Some(template_service.render_template(template_id, context)?)
        } else {
            None
        };

        let message_contents = EmailContents::new(subject, html, text);

        Ok(message_contents)
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use notifier::{template::Markup, Notification, Notifier};
    use serde::{Deserialize, Serialize};

    use super::{provider::test::TestProvider, *};

    #[derive(Serialize, Deserialize)]
    pub struct HelloNotification {
        name: String,
    }

    impl HelloNotification {
        pub fn new(name: String) -> Self {
            Self { name }
        }
    }

    impl Notification for HelloNotification {
        type Id = &'static str;

        fn id() -> Self::Id {
            "foo_notification"
        }
    }

    fn create_notifier(provider: TestProvider) -> Notifier<&'static str> {
        let mut notifier = Notifier::new();

        let mut channel = EmailChannel::new(
            provider,
            Options::new(
                EmailAddress::new("sender@test.com", None),
                Some(EmailAddress::new("reply-to@test.com", None)),
            ),
        );

        let email_template = EmailTemplate {
            html: Markup::Mjml(indoc! {r#"
                <mjml>
                    <mj-body>
                        <mj-section>
                            <mj-column>
                                <mj-text>
                                    Hello, {{ name }}!
                                </mj-text>
                            </mj-column>
                        </mj-section>
                    </mj-body>
                </mjml>
            "#}),
            subject: "Hello, {{ name }}!",
            text: Some("Hello, {{ name }}!"),
        };

        notifier.register_channel(channel);

        notifier
            .register_notification::<HelloNotification, EmailTemplate>(email_template)
            .unwrap();

        notifier
    }

    // fn create_message(channel: &EmailChannel) -> EmailMessage {
    //     let recipient = EmailAddress::new("recipient@test.com", None);

    //     let notification = HelloNotification::new("World".to_owned());

    //     channel.create_message(&notification, recipient).unwrap()
    // }

    #[tokio::test]
    async fn test_sets_default_sender() {
        let provider = TestProvider::default();
        let notifier = create_notifier(provider.clone());

        let contact = EmailAddress::new("recipient@test.com", None);
        let notification = HelloNotification::new("World".to_owned());

        notifier
            .send_message_to_contact(notification, contact)
            .await
            .unwrap();

        let message = provider
            .0
            .lock()
            .unwrap()
            .pop()
            .expect("message wasn't added to test provider correctly");

        assert_eq!(message.from(), &EmailAddress::new("sender@test.com", None));
    }

    #[tokio::test]
    async fn test_sets_default_reply_to() {
        let provider = TestProvider::default();
        let notifier = create_notifier(provider.clone());

        let contact = EmailAddress::new("recipient@test.com", None);
        let notification = HelloNotification::new("World".to_owned());

        notifier
            .send_message_to_contact(notification, contact)
            .await
            .unwrap();

        let message = provider
            .0
            .lock()
            .unwrap()
            .pop()
            .expect("message wasn't added to test provider correctly");

        assert_eq!(
            message.reply_to(),
            Some(&EmailAddress::new("reply-to@test.com", None))
        );
    }

    #[tokio::test]
    async fn test_renders_email_message() {
        let provider = TestProvider::default();
        let notifier = create_notifier(provider.clone());

        let contact = EmailAddress::new("recipient@test.com", None);
        let notification = HelloNotification::new("World".to_owned());

        notifier
            .send_message_to_contact(notification, contact)
            .await
            .unwrap();

        let message = provider
            .0
            .lock()
            .unwrap()
            .pop()
            .expect("message wasn't added to test provider correctly");

        assert_eq!(message.contents().subject(), "Hello, World!");
        assert_eq!(message.contents().text(), Some(&"Hello, World!".to_owned()));

        let html_contents = include_str!("../snapshots/expected_html_output.txt");
        pretty_assertions::assert_eq!(html_contents, message.contents().html());
    }
}
