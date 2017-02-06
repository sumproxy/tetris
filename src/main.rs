// Copyright 2014 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//pub type ColorFormat = gfx::format::Rgba8;
//pub type DepthFormat = gfx::format::DepthStencil;

#[macro_use]
extern crate gfx;
extern crate gfx_app;
extern crate bit_vec;

use gfx_app::ColorFormat;

use gfx::Bundle;

use bit_vec::BitVec;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "pos",
    }

    pipeline pipe {
        center: gfx::Global<[f32; 2]> = "u_center",
        color: gfx::Global<[f32; 3]> = "u_color",
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out_color: gfx::RenderTarget<ColorFormat> = "target",
    }
}

pub struct Screen {
    pub clear_color: [f32; 4],
    pub elem: Quad,
    frame: Frame,
}

impl Screen {
    pub fn new() -> Self {
        let elem = Quad::new();
        let frame = Frame::new(10, 22);
        let clear_color = [0.1, 0.1, 0.1, 1.0];
        let mut screen = Screen { clear_color: clear_color, elem: elem, frame: frame };
        let width = 1.0 / screen.frame.x() as f32;
        let height = 1.0 / screen.frame.y() as f32;
        screen.elem.set_vertices(width, height);
        screen
    }
    
    pub fn width(&self) -> u32 {
        self.elem.size * self.frame.x() as u32
    }

    pub fn height(&self) -> u32 {
        self.elem.size * self.frame.y() as u32
    }
}

pub struct Quad {
    pub vertices: [Vertex; 4],
    pub indices: [u16; 6],
    size: u32,
}

impl Quad {
    fn new() -> Self {
        Quad {
            vertices: [Vertex { pos: [0.0, 0.0] } ; 4],
            indices: [0, 1, 2, 1, 2, 3],
            size: 20,
        }
    }
    fn set_vertices(&mut self, width: f32, height: f32) {
        self.vertices = [
            Vertex { pos: [-width, -height] },
            Vertex { pos: [-width,  height] },
            Vertex { pos: [ width, -height] },
            Vertex { pos: [ width,  height] },
        ]
    }
}

struct Frame {
    inner: Vec<BitVec>
}

impl Frame {
    fn new(x: usize, y: usize) -> Self {
        let zeroes = BitVec::from_fn(x, |_| { false });
        Frame {
            inner: vec![zeroes; y],
        }
    }

    fn x(&self) -> usize {
        self.inner[0].len()
    }

    fn y(&self) -> usize {
        self.inner.len()
    }
}

struct App<R: gfx::Resources>{
    bundle: Bundle<R, pipe::Data<R>>,
    screen: Screen,
}

impl<R: gfx::Resources> gfx_app::Application<R> for App<R> {
    fn new<F: gfx::Factory<R>>(factory: &mut F, backend: gfx_app::shade::Backend, window_targets: gfx_app::WindowTargets<R>) -> Self {
        use gfx::traits::FactoryExt;

        let vs = gfx_app::shade::Source {
            glsl_150: include_bytes!("shader/tetris_150.glslv"),
            .. gfx_app::shade::Source::empty()
        };
        let ps = gfx_app::shade::Source {
            glsl_150: include_bytes!("shader/tetris_150.glslf"),
            .. gfx_app::shade::Source::empty()
        };

        let screen = Screen::new();
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&screen.elem.vertices, &screen.elem.indices as &[u16]);
        let data = pipe::Data {
            color: [1.0, 0.0, 0.0],
            center: [0.0, 0.0],
            vbuf: vertex_buffer,
            out_color: window_targets.color,
        };

    // App { window: window,
    //       encoder: encoder,
    //       device: device,
    //       slice: slice,
    //       pso: pso,
    //       data: data,
    //       screen: screen,
    //       state: GameState::Running,
    // }

        let pso = factory.create_pipeline_simple(
            vs.select(backend).unwrap(),
            ps.select(backend).unwrap(),
            pipe::new()
        ).unwrap();

        App {
            bundle: Bundle::new(slice, pso, data),
            screen: screen,
        }
    }

    fn render<C: gfx::CommandBuffer<R>>(&mut self, encoder: &mut gfx::Encoder<R, C>) {
        encoder.clear(&self.bundle.data.out_color, [0.1, 0.1, 0.1, 1.0]);
        self.bundle.encode(encoder);
    }

    fn on_resize(&mut self, window_targets: gfx_app::WindowTargets<R>) {
        self.bundle.data.out_color = window_targets.color;
    }
}

pub fn main() {
    use gfx_app::Application;
    App::launch_simple("Tetris");
}
