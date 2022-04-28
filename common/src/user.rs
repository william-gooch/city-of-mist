use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: i32,

    pub email: String,
    pub username: String,
}

#[cfg(feature = "db")]
use entity::user;

#[cfg(feature = "db")]
impl From<user::Model> for User {
    fn from(entity: user::Model) -> Self {
        Self {
            id: entity.id,
            email: entity.email,
            username: entity.username,
        }
    }
}
