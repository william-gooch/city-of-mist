use crate::schema::*;
use common::{Id, models::theme::*};

#[derive(PartialEq, Clone, Debug, Queryable, AsChangeset)]
#[diesel(table_name = themes)]
pub struct ThemeDbModel {
    pub id: Id,

    pub theme_descriptor: String,

    pub title: String,
    pub key_phrase: String,

    pub theme_type: ThemeType,

    pub attention: u8,
    pub degrade: u8,
    pub tags: serde_json::Value,
}

#[derive(PartialEq, Clone, Debug, Insertable)]
#[diesel(table_name = themes)]
pub struct ThemeInsertModel {
    pub theme_descriptor: String,

    pub title: String,
    pub key_phrase: String,

    pub theme_type: ThemeType,

    pub attention: u8,
    pub degrade: u8,
    pub tags: serde_json::Value,
}

impl From<NewTheme> for ThemeInsertModel {
    fn from(new_theme: NewTheme) -> ThemeInsertModel {
        ThemeInsertModel {
            theme_descriptor: new_theme.theme_descriptor,
            title: new_theme.title,
            key_phrase: new_theme.mystery_or_identity,
            theme_type: new_theme.theme_type,
            attention: new_theme.attention,
            degrade: new_theme.fade_or_crack,
            tags: serde_json::to_value(new_theme.tags).unwrap(),
        }
    }
}

impl From<Theme> for ThemeDbModel {
    fn from(theme: Theme) -> ThemeDbModel {
        ThemeDbModel {
            id: theme.id,
            theme_descriptor: theme.theme_descriptor,
            title: theme.title,
            key_phrase: theme.mystery_or_identity,
            theme_type: theme.theme_type,
            attention: theme.attention,
            degrade: theme.fade_or_crack,
            tags: serde_json::to_value(theme.tags).unwrap(),
        }
    }
}

impl TryInto<Theme> for ThemeDbModel {
    type Error = String;

    fn try_into(self) -> Result<Theme, String> {
        Ok(Theme {
            id: self.id,
            theme_descriptor: self.theme_descriptor,
            title: self.title,
            mystery_or_identity: self.key_phrase,
            theme_type: self.theme_type,
            attention: self.attention,
            fade_or_crack: self.degrade,
            tags: serde_json::from_value(self.tags).map_err(|_| "Couldn't deserialize 'tags' field.")?,
        })
    }
}
