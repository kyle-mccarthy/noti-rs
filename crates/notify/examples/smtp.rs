use indoc::indoc;
use lettre::{transport::smtp::authentication::Credentials, AsyncSmtpTransport, Tokio1Executor};
use notify::{
    contact::PersonContact, notification::Notification, provider::smtp::SmtpProvider,
    template::email::EmailTemplate, Notify,
};
use serde::Serialize;

pub enum Notifications {
    NewAccountNotification,
}

#[derive(Serialize)]
pub struct NewAccountNotification {
    activation_url: String,
}

impl Notification for NewAccountNotification {
    type Id = Notifications;

    fn id() -> Self::Id {
        Notifications::NewAccountNotification
    }
}

const TEST_SMTP_USER: &str = "shania.runolfsdottir72@ethereal.email";
const TEST_SMTP_PASS: &str = "vxCUzUZZXEGJ2XtJPB";
const TEST_SMTP_HOST: &str = "smtp.ethereal.email";

#[tokio::main]
pub async fn main() {
    let mut notify = Notify::default();

    // create and register the SMTP provider
    let creds = Credentials::new(TEST_SMTP_USER.to_string(), TEST_SMTP_PASS.to_string());

    let transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(TEST_SMTP_HOST)
        .unwrap()
        .credentials(creds)
        .port(587)
        .build::<Tokio1Executor>();

    let mut smtp_provider = SmtpProvider::new(transport);
    smtp_provider.set_default_sender("no-reply@example.com".to_string());

    notify.register_provider(smtp_provider);

    // create and register the template
    let email_template = EmailTemplate {
        html: indoc! {r#"
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
            "#},
        subject: "Example New Account Notification",
        text: Some(
            "Hi, please verify your account by clicking the following link: {{ activation_url }}",
        ),
    };

    notify
        .register_template::<NewAccountNotification, EmailTemplate>(email_template)
        .unwrap();

    let contact = PersonContact::new("test@test.com".to_string(), None).into_contact();

    let notification = NewAccountNotification {
        activation_url: "https://example.com/activate?code=123".to_string(),
    };

    notify.send(&contact, notification).await.unwrap();
}
