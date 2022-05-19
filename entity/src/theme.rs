use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "themes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub theme_descriptor: ThemeDescriptor,

    pub title: String,
    pub mystery_or_identity: String,
    pub theme_type: ThemeType,

    pub attention: i8,
    pub fade_or_crack: i8,
    pub tags: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "theme_type")]
pub enum ThemeType {
    #[sea_orm(string_value = "mythos")]
    Mythos,

    #[sea_orm(string_value = "logos")]
    Logos,

    #[sea_orm(string_value = "crew")]
    Crew,

    #[sea_orm(string_value = "extra")]
    Extra,
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

impl Into<String> for ThemeDescriptor {
    fn into(self) -> String {
        match self {
            ThemeDescriptor::Adaptation => "adaptation",
            ThemeDescriptor::Bastion => "bastion",
            ThemeDescriptor::Divination => "divination",
            ThemeDescriptor::Expression => "expression",
            ThemeDescriptor::Mobility => "mobility",
            ThemeDescriptor::Relic => "relic",
            ThemeDescriptor::Subversion => "subversion",
            ThemeDescriptor::DefiningEvent => "defining_event",
            ThemeDescriptor::DefiningRelationship => "defining_relationship",
            ThemeDescriptor::Mission => "mission",
            ThemeDescriptor::Personality => "personality",
            ThemeDescriptor::Possessions => "possessions",
            ThemeDescriptor::Routine => "routine",
            ThemeDescriptor::Training => "training",
            ThemeDescriptor::Crew => "crew",
            ThemeDescriptor::Ally => "ally",
            ThemeDescriptor::BaseOfOperations => "base_of_operations",
            ThemeDescriptor::Ride => "ride",
        }
        .to_owned()
    }
}

impl From<String> for ThemeDescriptor {
    fn from(str: String) -> Self {
        match &str[..] {
            "adaptation" => ThemeDescriptor::Adaptation,
            "bastion" => ThemeDescriptor::Bastion,
            "divination" => ThemeDescriptor::Divination,
            "expression" => ThemeDescriptor::Expression,
            "mobility" => ThemeDescriptor::Mobility,
            "relic" => ThemeDescriptor::Relic,
            "subversion" => ThemeDescriptor::Subversion,
            "defining_event" => ThemeDescriptor::DefiningEvent,
            "defining_relationship" => ThemeDescriptor::DefiningRelationship,
            "mission" => ThemeDescriptor::Mission,
            "personality" => ThemeDescriptor::Personality,
            "possessions" => ThemeDescriptor::Possessions,
            "routine" => ThemeDescriptor::Routine,
            "training" => ThemeDescriptor::Training,
            "crew" => ThemeDescriptor::Crew,
            "ally" => ThemeDescriptor::Ally,
            "base_of_operations" => ThemeDescriptor::BaseOfOperations,
            "ride" => ThemeDescriptor::Ride,
            _ => panic!(),
        }
        .to_owned()
    }
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
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
