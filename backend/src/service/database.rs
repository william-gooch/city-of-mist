use common::sea_orm::DatabaseConnection;
use shaku::{Component, Interface};
use std::sync::Arc;

pub trait Db: Interface {
    fn get(&self) -> &DatabaseConnection;
}

#[derive(Component)]
#[shaku(interface = Db)]
pub struct DbImpl {
    db: Arc<DatabaseConnection>,
}

impl Db for DbImpl {
    fn get(&self) -> &DatabaseConnection {
        &self.db.as_ref()
    }
}
