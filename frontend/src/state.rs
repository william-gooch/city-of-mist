use common::character::*;
use yewdux::prelude::*;

#[derive(Clone, PartialEq)]
pub struct State {
    pub character: Option<Character>,
}

impl Default for State {
    fn default() -> Self {
        Self { character: None }
    }
}
