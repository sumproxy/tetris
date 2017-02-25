use gfx;
use gfx::Bundle;
use gfx_app;
use gfx_app::ColorFormat;

use common::State;

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

        let state = State::new();
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&state.elem.vertices, &state.elem.indices as &[u16]);
        let data = pipe::Data {
            color: [1.0, 0.0, 0.0],
            center: [0.0, 0.0],
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
        data.center = [-0.5, 0.5];
        encoder.draw(&self.bundle.slice, &self.bundle.pso, &data);
        encoder.clear(&self.bundle.data.out_color, self.bundle.data.clear_color);
        self.bundle.encode(encoder);
    }
}
