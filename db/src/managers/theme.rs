use common::{
    managers::theme::ThemeManager,
    models::{
        character::*,
        theme::*,
    },
};
use shaku::Component;
use std::sync::Arc;
use diesel::prelude::*;
use crate::schema::characters;
use crate::schema::character_themes;
use crate::schema::themes;
use crate::Db;
use crate::models::{character::*, theme::*};

#[derive(Component)]
#[shaku(interface = ThemeManager)]
pub struct ThemeManagerImpl {
    #[shaku(inject)]
    db: Arc<dyn Db>,
}


impl ThemeManager for ThemeManagerImpl {
    fn create_theme(&self, new_theme: NewTheme) -> Theme {
        diesel::insert_into(themes::table)
            .values(&ThemeInsertModel::from(new_theme))
            .execute(&mut *self.db.connection())
            .unwrap();

        themes::table
            .filter(themes::id.eq(crate::last_insert_id()))
            .select(themes::all_columns)
            .first::<ThemeDbModel>(&mut *self.db.connection())
            .unwrap()
            .try_into()
            .unwrap()
    }

    fn add_theme_to_character(&self, character: &Character, theme: &Theme) -> () {
        diesel::insert_into(character_themes::table)
            .values(&(character_themes::character_id.eq(character.id), character_themes::theme_id.eq(theme.id)))
            .execute(&mut *self.db.connection())
            .unwrap();
    }
}
