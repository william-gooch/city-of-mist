use crate::theme::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Default, Serialize, Deserialize, Builder)]
pub struct Character {
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(into))]
    pub mythos: String,
    #[builder(setter(into))]
    pub logos: String,

    #[builder(default)]
    pub core_themes: Vec<Theme>,
    #[builder(default)]
    pub crew_theme: Theme,
    #[builder(default)]
    pub extra_themes: Vec<Theme>,
}
