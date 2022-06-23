#![feature(entry_insert)]

mod components;
mod console_writer;
mod queries;
mod routes;
mod services;
mod styles;
mod types;

use crate::{components::app::App, console_writer::WASMConsoleWriter};
use std::sync::Mutex;

#[cfg(test)]
use wasm_bindgen_test::*;


pub const BASE_URL : &'static str = base_url();

const fn base_url() -> &'static str {
    let base_url = "https://poetshuffle.com/";
    #[cfg(test)]
    let base_url = "http://127.0.0.1:3000/";
    base_url
}

pub const MSG_DURATION: u8 = 4;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
fn main() {
    console_error_panic_hook::set_once();
    // This subscriber just writes everything it hears to the console.
    tracing_subscriber::fmt()
        .without_time()
        .with_ansi(false)
        .with_writer(Mutex::new(WASMConsoleWriter))
        .pretty()
        .init();
    tracing::error!("main");
    yew::start_app::<App>();
}

#[cfg_attr(test, wasm_bindgen_test)]
fn pass() {
    assert!(true);
}
