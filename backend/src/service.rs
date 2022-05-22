use shaku::module;

pub mod auth;
pub mod character;
pub mod database;
pub mod rooms;
pub mod websocket;

pub use auth::Auth;
pub use character::CharacterManager;
pub use database::Db;
pub use rooms::Rooms;
pub use websocket::WsHandler;

module! {
    pub ServiceModule {
        components = [database::DbImpl, auth::AuthImpl, rooms::RoomsImpl, character::CharacterManagerImpl, websocket::WsHandlerImpl],
        providers = []
    }
}
