use crate::schema::*;
use diesel::prelude::*;

#[derive(Clone, Debug, PartialEq, Queryable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserUpdateModel {
    pub id: u64,

    pub email: Option<String>,
    pub username: Option<String>,
}
