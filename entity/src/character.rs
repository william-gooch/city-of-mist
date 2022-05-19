use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "character")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub name: String,
    pub mythos: String,
    pub logos: String,
}

impl Related<super::theme::Entity> for Entity {
    fn to() -> RelationDef {
        super::character_theme::Relation::Theme.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::character_theme::Relation::Character.def().rev())
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
