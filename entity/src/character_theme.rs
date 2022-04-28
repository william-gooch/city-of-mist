use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "character_theme")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub character_id: i32,
    #[sea_orm(primary_key)]
    pub theme_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::character::Entity",
        from = "Column::CharacterId",
        to = "super::character::Column::Id",
    )]
    Character,
    #[sea_orm(
        belongs_to = "super::theme::Entity",
        from = "Column::ThemeId",
        to = "super::theme::Column::Id",
    )]
    Theme,
}

impl ActiveModelBehavior for ActiveModel {}
