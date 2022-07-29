use crate::Id;

use super::theme_descriptor::ThemeDescriptor;
use derive_builder::Builder;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[cfg(feature = "db")]
use diesel_derive_enum::DbEnum;

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "db", derive(DbEnum))]
pub enum ThemeType {
    Mythos,
    Logos,
    Crew,
    Extra,
}

impl Default for ThemeType {
    fn default() -> Self {
        Self::Extra
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, new)]
pub enum Tag {
    Power { name: String, burned: bool },
    Weakness { name: String, invoked: bool },
    Story { name: String },
    Status { name: i8 },
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default, Builder, Getters)]
#[cfg_attr(feature = "db", derive(Queryable))]
#[builder(derive(Deserialize))]
pub struct Theme {
    pub id: Id,

    #[builder(setter(into))]
    pub theme_descriptor: String,

    #[builder(setter(into))]
    pub title: String,
    #[builder(setter(into))]
    pub mystery_or_identity: String,

    pub theme_type: ThemeType,

    #[builder(default)]
    pub attention: u8,
    #[builder(default)]
    pub fade_or_crack: u8,
    #[builder(default)]
    pub tags: Vec<Tag>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub struct NewTheme {
    pub theme_descriptor: String,

    pub title: String,
    pub mystery_or_identity: String,

    pub theme_type: ThemeType,

    pub attention: u8,
    pub fade_or_crack: u8,
    pub tags: Vec<Tag>,
}
