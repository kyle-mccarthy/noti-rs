pub mod error;
pub use error::Error;

use crate::message::Message;

#[async_trait::async_trait]
pub trait Provider: Sync + Send + 'static {
    type Message: Message;

    fn id(&self) -> &'static str;

    async fn send(&self, message: Self::Message) -> Result<(), Error>;
}

mod assertions {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct SomeMessage(String);

    static_assertions::assert_obj_safe!(Provider<Message = SomeMessage>);
}
