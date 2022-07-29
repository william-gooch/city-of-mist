use super::campaign::Campaign;
use serde::{Deserialize, Serialize};

#[cfg(feature = "db")]
use diesel::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "db", derive(Queryable))]
pub struct User {
    pub id: u64,

    pub email: String,
    pub username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "db", derive(Queryable))]
pub struct UserWithPassword {
    pub id: u64,

    pub email: String,
    pub username: String,

    pub password_hash: String,
    pub password_salt: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "db", derive(Queryable))]
pub struct UserWithCampaigns {
    pub id: u64,

    pub email: String,
    pub username: String,

    pub campaigns: Vec<Campaign>,
}
