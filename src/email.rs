pub struct Recipient {
    email: String,
    name: Option<String>,
}

pub struct Sender {
    email: String,
    name: Option<String>,
}

pub struct Email {
    to: Recipient,
    from: Option<Sender>,
    subject: String,
    html: String,
    text: Option<String>,
}

pub trait EmailTemplate {
    const HTML: &'static str;
    const TEXT: Option<&'static str> = None;
}

pub struct RegistrationNotification {
    user_id: u64,
}

impl EmailTemplate for RegistrationNotification {
    const HTML: &'static str = "<mrml>test</mrml>";
}
