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
