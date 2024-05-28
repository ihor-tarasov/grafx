grafx::impl_vertex! {
    struct Vertex {
        position: [f32; 3],
        color: [f32; 3],
    }
}

const fn vertex(position: [f32; 3], color: [f32; 3]) -> Vertex {
    Vertex { position, color }
}

const VERTICES: &[Vertex] = &[
    vertex([-0.5, 0.5, 0.0], [1.0, 0.0, 0.0]),
    vertex([-0.5, -0.5, 0.0], [0.0, 1.0, 0.0]),
    vertex([0.5, -0.5, 0.0], [0.0, 0.0, 1.0]),
    vertex([0.5, 0.5, 0.0], [1.0, 1.0, 0.0]),
];

const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

struct Game {
    pipeline: grafx::Pipeline,
    vertex_buffer: grafx::Buffer,
    index_buffer: grafx::Buffer,
}

impl grafx::State for Game {
    fn new(ctx: &mut grafx::Context) -> Self {
        let shader = ctx.shader(std::fs::read_to_string("data/shader.wgsl").unwrap());
        let vertex_buffer = ctx.vertex_buffer(VERTICES);
        let index_buffer = ctx.index_buffer(INDICES);
        let pipeline = ctx.pipeline(grafx::PipelineBuilder::new(&shader).with_vertex::<Vertex>());
        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
        }
    }

    fn render(&self, frame: &mut grafx::Frame) {
        let mut pass = frame.pass(0.1, 0.2, 0.3, 1.0);
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), grafx::IndexFormat::Uint16);
        pass.draw_indexed(0..6, 0, 0..1);
    }
}

fn main() {
    grafx::run::<Game>();
}
