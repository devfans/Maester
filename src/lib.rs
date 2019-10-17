
extern crate wand;
#[macro_use] mod utils;

mod app;
mod span;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


pub fn start() {
    let app = wand::core::Application::new_with_canvas_id("canvas");
    app.draw();
}


