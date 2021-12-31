use twilio_async::{Twilio, TwilioJson, TwilioRequest};

use crate::{
    channel::{sms::Sms, ChannelType, Error},
    Provider,
};

pub struct TwilioProvider {
    instance: Twilio,
}

#[derive(Debug, thiserror::Error)]
#[error("Twilio response indicates an error")]
pub struct TwilioError {
    pub code: usize,
    pub message: String,
    pub status: usize,
}

#[async_trait::async_trait]
impl Provider for TwilioProvider {
    type Message = Sms;

    fn id(&self) -> &'static str {
        "twilio"
    }

    async fn send(&self, message: Self::Message) -> Result<(), Error> {
        let message = self
            .instance
            .send_msg(message.from(), message.to(), message.contents());

        let response = message.run().await.map_err(|e| Error::Send {
            source: anyhow::Error::new(e),
            channel_type: ChannelType::Sms,
            context: Some("Twilio provider failed to send the SMS message"),
            provider_id: self.id(),
        })?;

        match response {
            TwilioJson::Fail {
                code,
                message,
                status,
            } => {
                return Err(Error::Send {
                    source: TwilioError {
                        code,
                        message,
                        status,
                    }
                    .into(),
                    channel_type: ChannelType::Sms,
                    context: Some("Twilio failed to send the message"),
                    provider_id: self.id(),
                });
            }
            TwilioJson::Success(_payload) => {
                // TODO consider returning some sort of response?
            }
        }

        Ok(())
    }
}
