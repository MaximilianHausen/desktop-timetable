#![feature(async_closure)]

use leptos::*;
use log::debug;
use wasm_bindgen::prelude::*;

use crate::app::*;

pub mod app;
pub mod components;
pub mod routes;
pub mod types;

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);

    debug!("Hydrating");

    leptos::mount_to_body(|cx| {
        view! { cx, <App/> }
    });
}
