use crate::{models::user::User, Id};
use shaku::Interface;

pub trait UserManager: Interface {
    fn get_user_by_id(&self, id: Id) -> User;
}
