use crate::sea_orm::{DbBackend, Schema};
use entity::character;
use entity::character_theme;
use entity::theme;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220505_000001_create_themes_characters"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(DbBackend::Postgres);

        manager
            .create_table(schema.create_table_from_entity(character::Entity))
            .await?;

        manager
            .create_table(schema.create_table_from_entity(theme::Entity))
            .await?;

        manager
            .create_table(schema.create_table_from_entity(character_theme::Entity))
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(character_theme::Entity)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(sea_query::Table::drop().table(theme::Entity).to_owned())
            .await?;
        manager
            .drop_table(sea_query::Table::drop().table(character::Entity).to_owned())
            .await
    }
}
