use indoc::indoc;
use lettre::{transport::smtp::authentication::Credentials, AsyncSmtpTransport, Tokio1Executor};
use noti::{
    contact::Contact,
    email::{provider::SmtpProvider, Address, EmailChannel, EmailTemplate},
    notification::Notification,
    template::Markup,
    Noti,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewAccountNotification {
    activation_url: String,
}

impl Notification for NewAccountNotification {
    type Id = &'static str;

    fn id() -> Self::Id {
        "new_account"
    }
}

const TEST_SMTP_USER: &str = "shania.runolfsdottir72@ethereal.email";
const TEST_SMTP_PASS: &str = "vxCUzUZZXEGJ2XtJPB";
const TEST_SMTP_HOST: &str = "smtp.ethereal.email";

#[tokio::main]
pub async fn main() {
    let mut notify = Noti::default();

    // create and register the SMTP provider
    let creds = Credentials::new(TEST_SMTP_USER.to_string(), TEST_SMTP_PASS.to_string());

    let transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(TEST_SMTP_HOST)
        .unwrap()
        .credentials(creds)
        .port(587)
        .build::<Tokio1Executor>();

    let smtp_provider = SmtpProvider::new(transport);

    let mut channel = EmailChannel::new(smtp_provider);
    channel.set_default_sender(Address::new("no-reply@email.com".to_string(), None));

    notify.register_channel(channel);

    // create and register the template
    let email_template = EmailTemplate {
        html: Markup::MJML(indoc! {r#"
                <mjml>
                    <mj-body>
                        <mj-section>
                            <mj-column>
                                <mj-text>Hi, please verify your account by clicking the following link: </mj-text>
                                <mj-text><a href="{{ activation_url }}">{{ activation_url }}</a></mj-text>
                            </mj-column>
                        </mj-section>
                    </mj-body>
                </mjml>
            "#}),
        subject: "Example New Account Notification",
        text: Some(
            "Hi, please verify your account by clicking the following link: {{ activation_url }}",
        ),
    };

    notify
        .register_template(NewAccountNotification::id(), email_template)
        .unwrap();

    let to = Address::new("test@test.com".to_string(), None);
    let contact: Contact = to.into();

    let notification = NewAccountNotification {
        activation_url: "https://example.com/activate?code=123".to_string(),
    };

    let result = notify.send(contact, notification).await;

    assert!(result.is_ok());
}
