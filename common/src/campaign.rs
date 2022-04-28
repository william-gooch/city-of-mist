use crate::character::Character;
use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub enum Member {
    GM(User),
    Player(User, Character),
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Default, Debug)]
pub struct Campaign {
    pub name: String,
    pub members: Option<Vec<Member>>,
}

impl Campaign {
    pub fn members(&self) -> &Option<Vec<Member>> {
        &self.members
    }

    pub fn members_mut(&mut self) -> &mut Option<Vec<Member>> {
        &mut self.members
    }
}
