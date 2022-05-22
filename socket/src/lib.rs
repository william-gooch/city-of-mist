#![feature(associated_type_defaults)]

#[cfg(feature = "ws")]
pub mod ws_socket;
#[cfg(feature = "ws")]
pub use ws_socket as ws;

#[cfg(feature = "js")]
pub mod js_socket;
#[cfg(feature = "js")]
pub use js_socket as js;
