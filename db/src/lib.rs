#[macro_use]
extern crate diesel;

pub mod managers;
pub mod models;
mod schema;

use diesel::prelude::*;
use diesel::MysqlConnection;
use dotenv::dotenv;
use parking_lot::{Mutex, MutexGuard};
use shaku::{Component, Interface};
use std::env;

sql_function!(fn last_insert_id() -> Unsigned<Bigint>);

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub trait Db: Interface {
    fn connection<'a>(&'a self) -> MutexGuard<'a, MysqlConnection>;
}

#[derive(Component)]
#[shaku(interface = Db)]
pub struct DbImpl {
    conn: Mutex<MysqlConnection>,
}

impl Db for DbImpl {
    fn connection<'a>(&'a self) -> MutexGuard<'a, MysqlConnection> {
        self.conn.lock()
    }
}
