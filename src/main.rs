#[macro_use]
extern crate gfx;
extern crate gfx_app;
extern crate winit;
extern crate rand;

mod app;
mod common;

use app::App;
use common::{color, template};

use winit::WindowBuilder;

fn main() {
    use gfx_app::Application;

    let wb = WindowBuilder::new()
        .with_min_dimensions(200, 440)
        .with_max_dimensions(200, 440)
        .with_title("Tetris!");

    App::launch_default(wb);
}
