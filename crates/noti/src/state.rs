use crate::{template, notification, Channels, Id};

#[derive(Default)]
pub struct State<'a, N: Id> {
    templates: template::Engine<'a>,
    notifications: notification::Store<N>,
    channels: Channels,
}
