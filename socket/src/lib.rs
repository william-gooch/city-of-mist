#![feature(associated_type_defaults)]

use async_trait::async_trait;
use std::ops::Deref;

#[cfg(feature = "ws")]
mod ws_socket;
#[cfg(feature = "ws")]
pub use ws_socket::*;

#[cfg(feature = "js")]
mod js_socket;
#[cfg(feature = "js")]
pub use js_socket::*;

use futures::Future;
use std::pin::Pin;
use std::sync::Arc;

type EventHandlerFunc<T> = dyn (Fn(T) -> Pin<Box<dyn Future<Output = ()> + Send>>) + Send + Sync;

pub struct EventHandler<T> {
    callback: Arc<EventHandlerFunc<T>>,
}

impl<T> EventHandler<T> {
    pub fn wrap<F, G>(func: F) -> Self
    where
        F: (Fn(T) -> G) + Send + Sync + 'static,
        G: Future<Output = ()> + Send + 'static,
    {
        let callback: Arc<EventHandlerFunc<T>> = Arc::new(move |args| {
            let future: Pin<Box<dyn Future<Output = ()> + Send>> = Box::pin(func(args));
            future
        });

        EventHandler { callback }
    }
}

impl<T> std::ops::Deref for EventHandler<T> {
    type Target = EventHandlerFunc<T>;

    fn deref(&self) -> &Self::Target {
        &*self.callback
    }
}

#[async_trait]
pub trait Socket {
    type EventType: Clone;
    type EventContext: Clone;
    type EventArgs: Clone;
    type EventHandler: Deref<Target = EventHandlerFunc<(Self::EventContext, Self::EventArgs)>>;

    fn on(&self, event_type: Self::EventType, callback: Self::EventHandler);
    async fn emit(&self, event_type: Self::EventType, event_args: Self::EventArgs);
}
