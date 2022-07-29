#[cfg(feature = "db")]
#[macro_use]
pub extern crate diesel;

pub mod managers;
pub mod models;

pub type Id = u64;
