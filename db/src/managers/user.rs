use diesel::prelude::*;
use shaku::{Component, Interface};
use std::sync::Arc;

use crate::schema::users;
use crate::Db;

use common::managers::user::UserManager;
use common::models::user::User;

#[derive(Component)]
#[shaku(interface = UserManager)]
pub struct UserManagerImpl {
    #[shaku(inject)]
    db: Arc<dyn Db>,
}

impl UserManager for UserManagerImpl {
    fn get_user_by_id(&self, id: u64) -> User {
        users::table
            .filter(users::id.eq(id))
            .select((users::id, users::email, users::username))
            .first::<User>(&mut *self.db.connection())
            .expect("Couldn't load user!")
    }
}
