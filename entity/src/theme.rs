use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "themes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub theme_descriptor: ThemeDescriptor,

    pub title: String,
    pub mystery_or_identity: String,

    pub attention: i8,
    pub fade_or_crack: i8,
    pub tags: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "theme_descriptor")]
pub enum ThemeDescriptor {
    #[sea_orm(string_value = "adaptation")]
    Adaptation,

    #[sea_orm(string_value = "bastion")]
    Bastion,

    #[sea_orm(string_value = "divination")]
    Divination,

    #[sea_orm(string_value = "expression")]
    Expression,

    #[sea_orm(string_value = "mobility")]
    Mobility,

    #[sea_orm(string_value = "relic")]
    Relic,

    #[sea_orm(string_value = "subversion")]
    Subversion,

    #[sea_orm(string_value = "defining_event")]
    DefiningEvent,

    #[sea_orm(string_value = "defining_relationship")]
    DefiningRelationship,

    #[sea_orm(string_value = "mission")]
    Mission,

    #[sea_orm(string_value = "personality")]
    Personality,

    #[sea_orm(string_value = "possessions")]
    Possessions,

    #[sea_orm(string_value = "routine")]
    Routine,

    #[sea_orm(string_value = "training")]
    Training,

    #[sea_orm(string_value = "crew")]
    Crew,

    #[sea_orm(string_value = "ally")]
    Ally,

    #[sea_orm(string_value = "base_of_operations")]
    BaseOfOperations,

    #[sea_orm(string_value = "ride")]
    Ride,
}

impl Related<super::character::Entity> for Entity {
    fn to() -> RelationDef {
        super::character_theme::Relation::Character.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::character_theme::Relation::Theme.def().rev())
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}
