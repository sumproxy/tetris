use gfx;
use gfx::Bundle;
use gfx_app;
use gfx_app::ColorFormat;
use winit::{Event, ElementState, VirtualKeyCode};

use common::State;
use common::template::DeltaPos;
use color;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "pos",
    }

    pipeline pipe {
        color: gfx::Global<[f32; 3]> = "u_color",
        center: gfx::Global<[f32; 2]> = "u_center",
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out_color: gfx::RenderTarget<ColorFormat> = "target",
        clear_color: gfx::Global<[f32; 4]> = "color",
    }
}

pub struct App<R: gfx::Resources>{
    bundle: Bundle<R, pipe::Data<R>>,
    state: State,
}

impl<R: gfx::Resources> gfx_app::Application<R> for App<R> {
    fn new<F: gfx::Factory<R>>(factory: &mut F,
                               backend: gfx_app::shade::Backend,
                               window_targets: gfx_app::WindowTargets<R>) -> Self {
        use gfx::traits::FactoryExt;

        let vs = gfx_app::shade::Source {
            glsl_150: include_bytes!("shader/tetris_150.glslv"),
            .. gfx_app::shade::Source::empty()
        };
        let ps = gfx_app::shade::Source {
            glsl_150: include_bytes!("shader/tetris_150.glslf"),
            .. gfx_app::shade::Source::empty()
        };

        let state = State::new();
        let width = state.box_width();
        let height = state.box_height();

        let vertices = [
            Vertex { pos: [-width, -height] },
            Vertex { pos: [-width,  height] },
            Vertex { pos: [ width, -height] },
            Vertex { pos: [ width,  height] },
        ];
        let indices = [0u16, 1, 2, 1, 2, 3];
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertices, &indices as &[u16]);

        let data = pipe::Data {
            color: color::Color::default().into(),
            center: [-2.0, -2.0],
            vbuf: vertex_buffer,
            out_color: window_targets.color,
            clear_color: [0.1, 0.1, 0.1, 1.0],
        };

        let pso = factory.create_pipeline_simple(
            vs.select(backend).unwrap(),
            ps.select(backend).unwrap(),
            pipe::new()
        ).unwrap();

        App {
            bundle: Bundle::new(slice, pso, data),
            state: state,
        }
    }

    fn render<C: gfx::CommandBuffer<R>>(&mut self, encoder: &mut gfx::Encoder<R, C>) {
        let mut data = self.bundle.data.clone();
        let box_width = self.state.box_width();
        let box_height = self.state.box_height();
        let middle_y = self.state.dim().h as f32 / 2.0 - 0.5;
        let middle_x = self.state.dim().w as f32 / 2.0 - 0.5;
        encoder.clear(&data.out_color, data.clear_color);
        for pos in self.state.inner.get_iter() {
            let x =   (pos.x as f32 / middle_x - 1.0) * (1.0 - box_width);
            let y = - (pos.y as f32 / middle_y - 1.0) * (1.0 - box_height);
            data.center = [x, y];
            data.color = self.state.inner.tile(pos).into();
            encoder.draw(&self.bundle.slice, &self.bundle.pso, &data);
        }
        self.bundle.encode(encoder);
    }

    fn on(&mut self, event: Event) {
        match event {
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Left)) => {
                self.state.move_piece(DeltaPos { dx: -1, dy: 0 });
            },
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Right)) => {
                self.state.move_piece(DeltaPos { dx: 1, dy: 0 });
            },
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Up)) => {
                self.state.move_piece(DeltaPos { dx: 0, dy: -1 });
            },
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Down)) => {
                self.state.move_piece(DeltaPos { dx: 0, dy: 1 });
            },
            _ => (),
        }
    }
}
