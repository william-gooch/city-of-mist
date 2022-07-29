use common::models::{character::*, theme::*};

#[derive(Default, Clone, PartialEq)]
pub struct State {
    pub character: Option<Character>,
    pub themes: Vec<Theme>,
}
