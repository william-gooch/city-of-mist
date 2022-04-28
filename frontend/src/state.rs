use common::character::*;
use data::example_character;

#[derive(Clone)]
pub struct State {
    pub character: Character,
}

impl Default for State {
    fn default() -> Self {
        Self {
            character: example_character(),
        }
    }
}
