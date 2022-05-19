use async_session::{MemoryStore, Session, SessionStore};
use async_trait::async_trait;
use shaku::{Component, Interface};

#[async_trait]
pub trait Auth: Interface {
    async fn load_session(&self, token: String) -> Result<Option<Session>, async_session::Error>;
    async fn store_session(&self, session: Session)
        -> Result<Option<String>, async_session::Error>;
}

#[derive(Component)]
#[shaku(interface = Auth)]
pub struct AuthImpl {
    #[shaku(default = MemoryStore::new())]
    store: MemoryStore,
}

#[async_trait]
impl Auth for AuthImpl {
    async fn load_session(&self, token: String) -> Result<Option<Session>, async_session::Error> {
        self.store.load_session(token).await
    }

    async fn store_session(
        &self,
        session: Session,
    ) -> Result<Option<String>, async_session::Error> {
        self.store.store_session(session).await
    }
}
