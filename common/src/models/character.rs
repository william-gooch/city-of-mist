use crate::Id;

use super::theme::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Default, Serialize, Deserialize, Builder)]
#[cfg_attr(feature = "db", derive(Queryable))]
pub struct Character {
    pub id: Id,

    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(into))]
    pub mythos: String,
    #[builder(setter(into))]
    pub logos: String,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct NewCharacter {
    pub name: String,
    pub mythos: String,
    pub logos: String,
}

pub type CharacterWithThemes = (Character, Vec<Theme>);
