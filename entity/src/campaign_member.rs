use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "campaign_member")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub campaign_id: i32,
    #[sea_orm(primary_key)]
    pub user_id: i32,

    pub character_id: Option<i32>,
    pub member_type: MemberType,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(10))")]
pub enum MemberType {
    #[sea_orm(string_value = "gm")]
    GM,

    #[sea_orm(string_value = "player")]
    Player,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::campaign::Entity",
        from = "Column::CampaignId",
        to = "super::campaign::Column::Id"
    )]
    Campaign,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::character::Entity",
        from = "Column::CharacterId",
        to = "super::character::Column::Id"
    )]
    Character,
}

impl ActiveModelBehavior for ActiveModel {}
