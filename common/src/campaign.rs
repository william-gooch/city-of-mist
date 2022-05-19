use crate::character::Character;
use crate::user::User;
use serde::{Deserialize, Serialize};
use std::iter::FilterMap;

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
    pub fn members(&self) -> Option<&Vec<Member>> {
        self.members.as_ref()
    }

    pub fn members_mut(&mut self) -> Option<&mut Vec<Member>> {
        self.members.as_mut()
    }

    pub fn gm(&self) -> Option<&User> {
        self.members()?.iter().find_map(|member| {
            if let Member::GM(user) = member {
                Some(user)
            } else {
                None
            }
        })
    }

    pub fn gm_mut(&mut self) -> Option<&mut User> {
        self.members_mut()?.iter_mut().find_map(|member| {
            if let Member::GM(user) = member {
                Some(user)
            } else {
                None
            }
        })
    }

    pub fn players(&self) -> Option<impl Iterator<Item = (&User, &Character)>> {
        Some(self.members()?.iter().filter_map(|member| {
            if let Member::Player(user, character) = member {
                Some((user, character))
            } else {
                None
            }
        }))
    }

    pub fn players_mut(&mut self) -> Option<impl Iterator<Item = (&mut User, &mut Character)>> {
        Some(self.members_mut()?.iter_mut().filter_map(|member| {
            if let Member::Player(user, character) = member {
                Some((user, character))
            } else {
                None
            }
        }))
    }
}

#[cfg(feature = "db")]
use entity::campaign;
#[cfg(feature = "db")]
use entity::campaign_member;

#[cfg(feature = "db")]
impl<U, C> From<(campaign_member::MemberType, U, Option<C>)> for Member
where
    U: Into<User>,
    C: Into<Character>,
{
    fn from((member_type, user, character): (campaign_member::MemberType, U, Option<C>)) -> Self {
        match member_type {
            campaign_member::MemberType::GM => Member::GM(user.into()),
            campaign_member::MemberType::Player => {
                Member::Player(user.into(), character.unwrap().into())
            }
        }
    }
}

#[cfg(feature = "db")]
impl From<campaign::Model> for Campaign {
    fn from(campaign: campaign::Model) -> Self {
        Self {
            name: campaign.name,

            ..Campaign::default()
        }
    }
}

#[cfg(feature = "db")]
impl<M> From<(campaign::Model, Vec<M>)> for Campaign
where
    M: Into<Member>,
{
    fn from((campaign, members): (campaign::Model, Vec<M>)) -> Self {
        Self {
            name: campaign.name,
            members: Some(members.into_iter().map(|member| member.into()).collect()),
        }
    }
}
