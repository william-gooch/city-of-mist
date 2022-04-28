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
pub struct Theme {
    #[builder(setter(into))]
    pub theme_descriptor: String,

    #[builder(setter(into))]
    pub title: String,
    #[builder(setter(into))]
    pub mystery_or_identity: String,

    #[builder(default)]
    pub attention: i8,
    #[builder(default)]
    pub fade_or_crack: i8,
    #[builder(default)]
    pub tags: Vec<Tag>,
}
