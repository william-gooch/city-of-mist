use shaku::module;

pub mod auth;
pub mod character;
pub mod database;
pub mod rooms;

pub use auth::Auth;
pub use character::CharacterManager;
pub use database::Db;
pub use rooms::Rooms;

module! {
    pub ServiceModule {
        components = [database::DbImpl, auth::AuthImpl, rooms::RoomsImpl, character::CharacterManagerImpl],
        providers = []
    }
}
