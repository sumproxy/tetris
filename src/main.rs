#[macro_use]
extern crate gfx;
extern crate gfx_app;
extern crate winit;
extern crate rand;

mod app;
mod state;

const BOX_SIZE: usize = 20;

fn main() {
    use gfx_app::Application;
    use winit::WindowBuilder;

    let width = ((state::MAIN_WIDTH + state::PREVIEW_WIDTH) * BOX_SIZE) as u32;
    let height = (state::HEIGHT * BOX_SIZE) as u32;
    let wb = WindowBuilder::new()
        .with_min_dimensions(width, height)
        .with_max_dimensions(width, height)
        .with_title("Tetris!");

    app::App::launch_default(wb);
}
