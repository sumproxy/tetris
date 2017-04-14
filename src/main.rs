#[macro_use]
extern crate gfx;
extern crate gfx_app;
extern crate winit;

mod app;
mod common;

use common::color;
use common::piece;
use common::map;

use winit::WindowBuilder;
use app::App;

fn main() {
    use gfx_app::Application;

    let wb = WindowBuilder::new()
        .with_min_dimensions(200, 440)
        .with_max_dimensions(200, 440)
        .with_title("Tetris!");

    App::launch_default(wb);

    let i = piece::J;
    i.rotate_left();
}
