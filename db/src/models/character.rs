use diesel::prelude::*;

use crate::schema::*;
use common::{Id, models::character::{Character, NewCharacter}};

#[derive(PartialEq, Clone, Debug, Default, Queryable, AsChangeset)]
#[diesel(table_name = characters)]
pub struct CharacterDbModel {
    pub id: Id,

    pub name: String,
    pub mythos: String,
    pub logos: String,
}

#[derive(PartialEq, Clone, Debug, Default, Insertable)]
#[diesel(table_name = characters)]
pub struct CharacterInsertModel {
    pub name: String,
    pub mythos: String,
    pub logos: String,
}

impl From<NewCharacter> for CharacterInsertModel {
    fn from(new_character: NewCharacter) -> CharacterInsertModel {
        CharacterInsertModel {
            name: new_character.name,
            mythos: new_character.mythos,
            logos: new_character.logos,
        }
    }
}

impl From<Character> for CharacterDbModel {
    fn from(character: Character) -> CharacterDbModel {
        CharacterDbModel {
            id: character.id,
            name: character.name,
            mythos: character.mythos,
            logos: character.logos,
        }
    }
}

impl Into<Character> for CharacterDbModel {
    fn into(self) -> Character {
        Character {
            id: self.id,
            name: self.name,
            mythos: self.mythos,
            logos: self.logos,
        }
    }
}
