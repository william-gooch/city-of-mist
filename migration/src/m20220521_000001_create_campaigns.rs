use crate::sea_orm::{DbBackend, Schema};
use entity::campaign;
use entity::campaign_member;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220521_000001_create_campaigns"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(DbBackend::Postgres);

        manager
            .create_table(schema.create_table_from_entity(campaign::Entity))
            .await?;

        manager
            .create_table(schema.create_table_from_entity(campaign_member::Entity))
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(campaign_member::Entity)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(sea_query::Table::drop().table(campaign::Entity).to_owned())
            .await
    }
}
