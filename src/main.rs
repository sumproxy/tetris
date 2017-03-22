#[macro_use]
extern crate gfx;
extern crate gfx_app;
extern crate winit;
extern crate bit_vec;

mod app;
mod common;

use winit::WindowBuilder;
use app::App;

fn main() {
    let wb = WindowBuilder::new()
        .with_min_dimensions(200, 440)
        .with_max_dimensions(200, 440)
        .with_title("Tetris!");
    use gfx_app::Application;
    App::launch_default(wb);
}
