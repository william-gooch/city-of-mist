use crate::theme_descriptor::ThemeDescriptor;
use derive_builder::Builder;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
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
#[builder(derive(Deserialize))]
pub struct Theme {
    #[builder(default)]
    pub id: Option<i32>,

    #[builder(setter(into))]
    pub theme_descriptor: String,

    #[builder(setter(into))]
    pub title: String,
    #[builder(setter(into))]
    pub mystery_or_identity: String,

    pub theme_type: ThemeType,

    #[builder(default)]
    pub attention: i8,
    #[builder(default)]
    pub fade_or_crack: i8,
    #[builder(default)]
    pub tags: Vec<Tag>,
}

#[cfg(feature = "db")]
use entity::sea_orm::ActiveValue::*;
#[cfg(feature = "db")]
use entity::theme;

#[cfg(feature = "db")]
impl From<theme::ThemeType> for ThemeType {
    fn from(theme_type: theme::ThemeType) -> Self {
        match theme_type {
            theme::ThemeType::Mythos => ThemeType::Mythos,
            theme::ThemeType::Logos => ThemeType::Logos,
            theme::ThemeType::Crew => ThemeType::Crew,
            theme::ThemeType::Extra => ThemeType::Extra,
        }
    }
}

#[cfg(feature = "db")]
impl Into<theme::ThemeType> for ThemeType {
    fn into(self: ThemeType) -> theme::ThemeType {
        match self {
            ThemeType::Mythos => theme::ThemeType::Mythos,
            ThemeType::Logos => theme::ThemeType::Logos,
            ThemeType::Crew => theme::ThemeType::Crew,
            ThemeType::Extra => theme::ThemeType::Extra,
        }
    }
}

#[cfg(feature = "db")]
impl From<theme::Model> for Theme {
    fn from(entity: theme::Model) -> Self {
        Self {
            id: Some(entity.id),
            theme_descriptor: entity.theme_descriptor.into(),

            title: entity.title,
            mystery_or_identity: entity.mystery_or_identity,
            theme_type: entity.theme_type.into(),

            attention: entity.attention,
            fade_or_crack: entity.fade_or_crack,
            tags: serde_json::from_value(entity.tags).unwrap(),
        }
    }
}

#[cfg(feature = "db")]
impl Into<(theme::ActiveModel,)> for Theme {
    fn into(self: Theme) -> (theme::ActiveModel,) {
        let theme_model = theme::ActiveModel {
            id: self.id.map_or_else(|| NotSet, Set),
            theme_descriptor: Set(self.theme_descriptor.into()),

            title: Set(self.title),
            mystery_or_identity: Set(self.mystery_or_identity),
            theme_type: Set(self.theme_type.into()),

            attention: Set(self.attention),
            fade_or_crack: Set(self.fade_or_crack),
            tags: Set(serde_json::to_value(self.tags).unwrap()),
        };

        (theme_model,)
    }
}

#[cfg(feature = "db")]
impl Into<(theme::ActiveModel,)> for ThemeBuilder {
    fn into(self: ThemeBuilder) -> (theme::ActiveModel,) {
        let theme_model = theme::ActiveModel {
            id: self.id.flatten().map_or_else(|| NotSet, Set),
            theme_descriptor: self
                .theme_descriptor
                .map_or_else(|| NotSet, |v| Set(v.into())),

            title: self.title.map_or_else(|| NotSet, Set),
            mystery_or_identity: self.mystery_or_identity.map_or_else(|| NotSet, Set),
            theme_type: self.theme_type.map_or_else(|| NotSet, |v| Set(v.into())),

            attention: self.attention.map_or_else(|| NotSet, Set),
            fade_or_crack: self.fade_or_crack.map_or_else(|| NotSet, Set),
            tags: self
                .tags
                .map_or_else(|| NotSet, |v| Set(serde_json::to_value(v).unwrap())),
        };

        (theme_model,)
    }
}
