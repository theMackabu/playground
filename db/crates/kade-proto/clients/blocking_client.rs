use bytes::Bytes;
use std::time::Duration;
use tokio::net::ToSocketAddrs;
use tokio::runtime::Runtime;

pub use crate::clients::Message;

pub struct BlockingClient {
    rt: Runtime,
    inner: crate::clients::Client,
}

pub struct BlockingSubscriber {
    rt: Runtime,
    inner: crate::clients::Subscriber,
}

struct SubscriberIterator {
    rt: Runtime,
    inner: crate::clients::Subscriber,
}

impl BlockingClient {
    pub fn connect<T: ToSocketAddrs>(addr: T) -> crate::Result<BlockingClient> {
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;

        let inner = rt.block_on(crate::clients::Client::connect(addr))?;

        Ok(BlockingClient { inner, rt })
    }

    pub fn get(&mut self, key: &str) -> crate::Result<Option<Bytes>> { self.rt.block_on(self.inner.get(key)) }

    pub fn set(&mut self, key: &str, value: Bytes) -> crate::Result<()> { self.rt.block_on(self.inner.set(key, value)) }

    pub fn set_expires(&mut self, key: &str, value: Bytes, expiration: Duration) -> crate::Result<()> { self.rt.block_on(self.inner.set_expires(key, value, expiration)) }

    pub fn publish(&mut self, channel: &str, message: Bytes) -> crate::Result<u64> { self.rt.block_on(self.inner.publish(channel, message)) }

    pub fn subscribe(self, channels: Vec<String>) -> crate::Result<BlockingSubscriber> {
        let subscriber = self.rt.block_on(self.inner.subscribe(channels))?;
        Ok(BlockingSubscriber { inner: subscriber, rt: self.rt })
    }
}

impl BlockingSubscriber {
    pub fn get_subscribed(&self) -> &[String] { self.inner.get_subscribed() }

    pub fn next_message(&mut self) -> crate::Result<Option<Message>> { self.rt.block_on(self.inner.next_message()) }

    pub fn into_iter(self) -> impl Iterator<Item = crate::Result<Message>> { SubscriberIterator { inner: self.inner, rt: self.rt } }

    pub fn subscribe(&mut self, channels: &[String]) -> crate::Result<()> { self.rt.block_on(self.inner.subscribe(channels)) }

    pub fn unsubscribe(&mut self, channels: &[String]) -> crate::Result<()> { self.rt.block_on(self.inner.unsubscribe(channels)) }
}

impl Iterator for SubscriberIterator {
    type Item = crate::Result<Message>;

    fn next(&mut self) -> Option<crate::Result<Message>> { self.rt.block_on(self.inner.next_message()).transpose() }
}
