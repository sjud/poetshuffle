
mod services;
mod console_writer;
mod queries;
mod components;
mod routes;
mod types;


use std::sync::Mutex;
use crate::{
    components::app::App,
    console_writer::WASMConsoleWriter
};


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
    yew::start_app::<App>();
}
