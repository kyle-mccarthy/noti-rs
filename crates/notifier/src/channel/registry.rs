use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::{
    channel::{ChannelType, DynChannel},
    Channel, Id,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Key {
    Message(TypeId),
    Contact(TypeId),
    Template(TypeId),
}

#[derive(Default)]
pub struct ChannelRegistry<I: Id> {
    channels: HashMap<ChannelType, Box<dyn DynChannel<I>>>,
    type_map: HashMap<Key, ChannelType>,
}

impl<I: Id> ChannelRegistry<I> {
    pub fn register<C: Channel<I>>(&mut self, channel: C) {
        let channel_type = channel.channel_type();
        let dyn_channel = channel.into_dyn();

        self.channels.insert(channel_type, dyn_channel);

        // types used for loop-ups
        self.type_map
            .insert(Key::Template(TypeId::of::<C::UserTemplate>()), channel_type);
        self.type_map
            .insert(Key::Message(TypeId::of::<C::Message>()), channel_type);
        self.type_map
            .insert(Key::Contact(TypeId::of::<C::Contact>()), channel_type);
    }

    pub fn find_by_template<T: Any>(&self) -> Option<&dyn DynChannel<I>> {
        let key = Key::Template(TypeId::of::<T>());

        if let Some(channel_type) = self.type_map.get(&key) {
            return self.get(*channel_type);
        }

        None
    }

    pub fn find_by_contact<T: Any>(&self) -> Option<&dyn DynChannel<I>> {
        let key = Key::Contact(TypeId::of::<T>());

        if let Some(channel_type) = self.type_map.get(&key) {
            return self.get(*channel_type);
        }

        None
    }

    // pub fn get_channel_by_message<M: M()

    pub fn get(&self, channel_type: ChannelType) -> Option<&dyn DynChannel<I>> {
        self.channels.get(&channel_type).map(|b| b.as_ref())
    }
}

#[cfg(test)]
mod test_channel_registry {
    use super::ChannelRegistry;
    use crate::{test_utils::TestChannel, Channel};

    #[test]
    fn test_register_and_get_channel() {
        let channel = TestChannel::default();
        let channel_type = <TestChannel as Channel<u8>>::channel_type(&channel);

        let mut registry = ChannelRegistry::<u8>::default();

        registry.register(channel);

        let dyn_channel = registry.get(channel_type);

        assert!(dyn_channel.is_some());
    }
}
