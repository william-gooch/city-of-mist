use crate::theme::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Default, Serialize, Deserialize, Builder)]
#[builder(derive(Deserialize))]
pub struct Character {
    #[builder(default)]
    pub id: Option<i32>,

    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(into))]
    pub mythos: String,
    #[builder(setter(into))]
    pub logos: String,

    #[builder(field(
        type = "Option<Vec<ThemeBuilder>>",
        build = "self.core_themes.as_ref().map(|themes| themes.iter().map(|theme| theme.build()).collect::<Result<Vec<Theme>, ThemeBuilderError>>()).transpose()?.unwrap_or_else(|| vec![])"
    ))]
    pub core_themes: Vec<Theme>,
    #[builder(field(
        type = "Option<Option<ThemeBuilder>>",
        build = "self.crew_theme.as_ref().map(|ct| ct.as_ref().map(|ct| ct.build())).flatten().transpose()?"
    ))]
    pub crew_theme: Option<Theme>,
    #[builder(field(
        type = "Option<Vec<ThemeBuilder>>",
        build = "self.extra_themes.as_ref().map(|themes| themes.iter().map(|theme| theme.build()).collect::<Result<Vec<Theme>, ThemeBuilderError>>()).transpose()?.unwrap_or_else(|| vec![])"
    ))]
    pub extra_themes: Vec<Theme>,
}

impl From<ThemeBuilderError> for CharacterBuilderError {
    fn from(err: ThemeBuilderError) -> Self {
        match err {
            ThemeBuilderError::UninitializedField(field) => {
                CharacterBuilderError::UninitializedField(field)
            }
            ThemeBuilderError::ValidationError(msg) => CharacterBuilderError::ValidationError(msg),
        }
    }
}

#[cfg(feature = "db")]
use entity::{character, sea_orm::ActiveValue::*, theme};

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

#[cfg(feature = "db")]
impl Into<(character::ActiveModel, Vec<theme::ActiveModel>)> for CharacterBuilder {
    fn into(self: Self) -> (character::ActiveModel, Vec<theme::ActiveModel>) {
        let character_model = character::ActiveModel {
            id: self.id.flatten().map_or_else(|| NotSet, Set),
            name: self.name.map_or_else(|| NotSet, Set),
            mythos: self.mythos.map_or_else(|| NotSet, Set),
            logos: self.logos.map_or_else(|| NotSet, Set),
        };

        let themes: Vec<theme::ActiveModel> = self
            .core_themes
            .unwrap_or_else(|| vec![])
            .iter()
            .chain(self.crew_theme.flatten().iter())
            .chain(self.extra_themes.unwrap_or_else(|| vec![]).iter())
            .map(|theme: &ThemeBuilder| -> (theme::ActiveModel,) { theme.clone().into() })
            .map(|t| t.0)
            .collect();

        (character_model, themes)
    }
}
