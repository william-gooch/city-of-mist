use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub email: String,
    pub username: String,

    pub password_hash: String,
    pub password_salt: String,
}

impl Related<super::campaign::Entity> for Entity {
    fn to() -> RelationDef {
        super::campaign_member::Relation::Campaign.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::campaign_member::Relation::User.def().rev())
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
