pub use notify_macros::EmailNotification;

pub mod email;
pub mod template;

pub type DateTime = chrono::DateTime<chrono::Utc>;

pub enum ChannelType {
    Email,
}

pub struct Notify {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
