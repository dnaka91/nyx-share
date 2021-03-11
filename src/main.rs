#![deny(rust_2018_idioms, clippy::all)]
#![recursion_limit = "512"]

mod app;
mod components;
mod languages;
mod multipart;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc<'_> = wee_alloc::WeeAlloc::INIT;

fn main() {
    console_error_panic_hook::set_once();
    yew::start_app::<app::App>();
}
