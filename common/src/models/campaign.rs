use crate::Id;

use super::{character::Character, user::User};
use serde::{Deserialize, Serialize};
use std::iter::FilterMap;

#[cfg(feature = "db")]
use diesel_derive_enum::DbEnum;

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "db", derive(DbEnum))]
pub enum MemberType {
    GM,
    Player,
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Member {
    member_type: MemberType,
    user: User,
    character: Option<Character>,
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Default, Debug)]
pub struct Campaign {
    pub id: Id,
    pub name: String,
    pub members: Option<Vec<Member>>,
}

impl Campaign {
    pub fn members(&self) -> Option<&Vec<Member>> {
        self.members.as_ref()
    }

    pub fn members_mut(&mut self) -> Option<&mut Vec<Member>> {
        self.members.as_mut()
    }
}
