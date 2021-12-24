use crate::channel::ChannelType;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("A contact does not exist with this id. (id = {0})")]
    ContactNotFound(String),

    #[error("A contact exists but is missign data for this field. (id = {0}, field = {1})")]
    MissingField(String, &'static str),
}

impl Error {
    pub fn is_missing_field(&self) -> bool {
        matches!(self, Error::MissingField(_, _))
    }
}
