use common::user::User;

use core::fmt::Debug;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{broadcast, broadcast::Sender, RwLock};
use tokio_stream::wrappers::BroadcastStream;
use warp::Filter;

pub type UserWatcher = Arc<WatcherService<User>>;
pub fn with_watcher(
    watcher: UserWatcher,
) -> impl Filter<Extract = (UserWatcher,), Error = Infallible> + Clone {
    warp::any().map(move || watcher.clone())
}

pub struct Watcher<T>
where
    T: 'static + Debug + Clone + Send,
{
    sender: Sender<T>,
}

impl<T> Clone for Watcher<T>
where
    T: 'static + Debug + Clone + Send,
{
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

impl<T> Watcher<T>
where
    T: 'static + Debug + Clone + Send,
{
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(16);
        Self { sender: tx }
    }

    pub fn watch(&self) -> BroadcastStream<T> {
        BroadcastStream::new(self.sender.subscribe())
    }

    pub fn trigger(&self, val: T) {
        self.sender.send(val).unwrap();
    }
}

pub struct WatcherService<T>
where
    T: 'static + Debug + Clone + Send,
{
    watchers: RwLock<HashMap<i32, Watcher<T>>>,
}

impl<T> WatcherService<T>
where
    T: 'static + Debug + Clone + Send,
{
    pub fn new() -> Self {
        Self {
            watchers: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get_by_id(&self, id: i32) -> Watcher<T> {
        let mut watchers = self.watchers.write().await;
        if let Some(watcher) = watchers.get_mut(&id) {
            watcher.clone()
        } else {
            let new_watcher = Watcher::<T>::new();
            watchers.insert(id, new_watcher.clone());
            new_watcher
        }
    }
}
