use crate::theme::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Default, Serialize, Deserialize, Builder)]
pub struct Character {
    #[builder(default)]
    pub id: Option<i32>,

    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(into))]
    pub mythos: String,
    #[builder(setter(into))]
    pub logos: String,

    #[builder(default)]
    pub core_themes: Vec<Theme>,
    #[builder(default)]
    pub crew_theme: Option<Theme>,
    #[builder(default)]
    pub extra_themes: Vec<Theme>,
}

#[cfg(feature = "db")]
use entity::character;
#[cfg(feature = "db")]
use entity::sea_orm::ActiveValue::*;
#[cfg(feature = "db")]
use entity::theme;

#[cfg(feature = "db")]
impl From<character::Model> for Character {
    fn from(character: character::Model) -> Self {
        Self {
            name: character.name,
            mythos: character.mythos,
            logos: character.logos,

            ..Character::default()
        }
    }
}

#[cfg(feature = "db")]
impl<T> From<(character::Model, Vec<T>)> for Character
where
    T: Into<Theme>,
{
    fn from((character, themes): (character::Model, Vec<T>)) -> Self {
        let themes = themes.into_iter().map(|theme| theme.into());

        let (core_themes, themes): (Vec<Theme>, Vec<Theme>) = themes.partition(|theme| {
            theme.theme_type == ThemeType::Mythos || theme.theme_type == ThemeType::Logos
        });

        let (extra_themes, themes): (Vec<Theme>, Vec<Theme>) = themes
            .into_iter()
            .partition(|theme| theme.theme_type == ThemeType::Extra);

        let crew_theme = themes
            .into_iter()
            .find(|theme| theme.theme_type == ThemeType::Crew);

        Self {
            id: Some(character.id),
            name: character.name,
            mythos: character.mythos,
            logos: character.logos,

            core_themes,
            crew_theme,
            extra_themes,
        }
    }
}

#[cfg(feature = "db")]
impl Into<(character::ActiveModel, Vec<theme::ActiveModel>)> for Character {
    fn into(self: Self) -> (character::ActiveModel, Vec<theme::ActiveModel>) {
        let character_model = character::ActiveModel {
            id: self.id.map_or_else(|| NotSet, Set),
            name: Set(self.name),
            mythos: Set(self.mythos),
            logos: Set(self.logos),
        };

        let themes: Vec<theme::ActiveModel> = self
            .core_themes
            .iter()
            .chain(self.crew_theme.iter())
            .chain(self.extra_themes.iter())
            .map(|theme| -> (theme::ActiveModel,) { theme.clone().into() })
            .map(|t| t.0)
            .collect();

        (character_model, themes)
    }
}
