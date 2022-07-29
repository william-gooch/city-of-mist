use shaku::Interface;

use crate::{models::character::*, Id};

pub trait CharacterManager: Interface {
    fn create_character(&self, new_character: NewCharacter) -> Character;
    fn get_character_by_id(&self, id: Id) -> Character;
    fn get_character_with_themes_by_id(&self, id: Id) -> CharacterWithThemes;
    fn update_character(&self, character: &Character) -> ();
    fn update_character_and_themes(&self, character: &CharacterWithThemes) -> ();
}
