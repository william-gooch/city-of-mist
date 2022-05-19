use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "campaigns")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub name: String,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        super::campaign_member::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::campaign_member::Relation::Campaign.def().rev())
    }
}

impl Related<super::character::Entity> for Entity {
    fn to() -> RelationDef {
        super::campaign_member::Relation::Character.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::campaign_member::Relation::User.def().rev())
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
