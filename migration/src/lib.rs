pub use sea_schema::migration::prelude::*;

mod m20220404_000001_create_users;
mod m20220504_000001_create_themes_characters;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220404_000001_create_users::Migration),
            Box::new(m20220504_000001_create_themes_characters::Migration),
        ]
    }
}
