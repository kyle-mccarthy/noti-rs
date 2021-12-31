use serde::Serialize;

use super::SmsBuilder;
use crate::{
    template::{Engine, RenderTemplate, TemplateId},
    RegisterTemplate,
};

pub struct SmsTemplate(&'static str);

impl RegisterTemplate for SmsTemplate {
    fn register<N: crate::Id>(
        self,
        notification_id: N,
        instance: &mut crate::Noti<N>,
    ) -> Result<(), crate::Error> {
        let contents = instance.templates.register(self.0)?;

        let template = RegisteredSmsTemplate(contents);

        instance
            .notifications
            .entry(notification_id)
            .or_default()
            .templates
            .set_sms(Some(template));

        Ok(())
    }
}

pub struct RegisteredSmsTemplate(TemplateId);

impl RenderTemplate for RegisteredSmsTemplate {
    type MessageBuilder = SmsBuilder;

    fn render<T: Serialize>(
        &self,
        engine: &Engine,
        data: &T,
    ) -> Result<Self::MessageBuilder, crate::template::Error> {
        let contents = engine.render(&self.0, data)?;

        let builder = SmsBuilder::new().contents(contents);

        Ok(builder)
    }
}
