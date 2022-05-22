#![feature(let_chains)]

mod components;
mod state;
mod utils;

use crate::components::app::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
