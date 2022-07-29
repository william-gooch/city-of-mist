use common::{
    managers::character::CharacterManager,
    models::{
        character::*,
        theme::{Theme},
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
#[shaku(interface = CharacterManager)]
pub struct CharacterManagerImpl {
    #[shaku(inject)]
    db: Arc<dyn Db>,
}

impl CharacterManager for CharacterManagerImpl {
    fn create_character(&self, new_character: NewCharacter) -> Character {
        diesel::insert_into(characters::table)
            .values(&CharacterInsertModel::from(new_character))
            .execute(&mut *self.db.connection())
            .unwrap();
        
        characters::table
            .filter(characters::id.eq(crate::last_insert_id()))
            .select((characters::id, characters::name, characters::mythos, characters::logos))
            .first::<Character>(&mut *self.db.connection())
            .unwrap()
    }

    fn get_character_by_id(&self, id: common::Id) -> Character {
        characters::table
            .filter(characters::id.eq(id))
            .select((characters::id, characters::name, characters::mythos, characters::logos))
            .first::<Character>(&mut *self.db.connection())
            .expect("Couldn't load character!")
    }

    fn get_character_with_themes_by_id(&self, id: common::Id) -> CharacterWithThemes {
        let (characters, themes): (Vec<CharacterDbModel>, Vec<ThemeDbModel>) = character_themes::table
            .inner_join(characters::table)
            .inner_join(themes::table)
            .filter(characters::id.eq(id))
            .select((
                (characters::id, characters::name, characters::mythos, characters::logos),
                (themes::all_columns)
            ))
            .load::<(CharacterDbModel, ThemeDbModel)>(&mut *self.db.connection())
            .expect("Couldn't load character!")
            .into_iter()
            .unzip();

        let character: Character = characters
            .into_iter()
            .next()
            .unwrap()
            .into();

        let themes = themes
            .into_iter()
            .map(|t| t.try_into())
            .collect::<Result<Vec<Theme>, String>>()
            .unwrap();

        (character, themes)
    }

    fn update_character(&self, character: &Character) -> () {
        diesel::update(characters::table)
            .set(CharacterDbModel::from(character.clone()))
            .execute(&mut *self.db.connection())
            .unwrap();
    }

    fn update_character_and_themes(&self, character_with_themes: &CharacterWithThemes) -> () {
        let (character, themes) = character_with_themes.clone();

        diesel::update(characters::table)
            .set(CharacterDbModel::from(character))
            .execute(&mut *self.db.connection())
            .unwrap();

        themes.into_iter().for_each(|theme| {
            diesel::update(themes::table)
                .set(ThemeDbModel::from(theme))
                .execute(&mut *self.db.connection())
                .unwrap();
        });
    }
}
