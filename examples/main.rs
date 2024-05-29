use glam::*;
use grafx::BindGroupBuilderWith;

grafx::impl_vertex! {
    struct Vertex {
        position: [f32; 3],
        color: [f32; 3],
        uv: [f32; 2],
    }
}

const fn vertex(position: [f32; 3], color: [f32; 3], uv: [f32; 2]) -> Vertex {
    Vertex {
        position,
        color,
        uv,
    }
}

const VERTICES: &[Vertex] = &[
    vertex([-0.5, 0.5, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0]),
    vertex([-0.5, -0.5, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0]),
    vertex([0.5, -0.5, 0.0], [0.0, 0.0, 1.0], [1.0, 1.0]),
    vertex([0.5, 0.5, 0.0], [1.0, 1.0, 0.0], [1.0, 0.0]),
];

const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

struct Camera {
    position: Vec3,
    target: Vec3,
}

impl Camera {
    fn new() -> Self {
        Self {
            position: vec3(0.0, 1.0, 2.0),
            target: vec3(0.0, 0.0, 0.0),
        }
    }

    fn view_projection(&self, width: f32, height: f32) -> Mat4 {
        let view = Mat4::look_at_rh(self.position, self.target, Vec3::Y);
        let projection = Mat4::perspective_rh(76f32.to_radians(), width / height, 0.1, 1000.0);
        projection * view
    }
}

grafx::impl_uniform! {
    struct CameraUniform {
        view_proj: [[f32; 4]; 4],
    }
}

impl CameraUniform {
    fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    fn update_view_proj(&mut self, camera: &Camera, width: f32, height: f32) {
        self.view_proj = camera.view_projection(width, height).to_cols_array_2d();
    }
}

struct Game {
    pipeline: grafx::Pipeline,
    vertex_buffer: grafx::Buffer,
    index_buffer: grafx::Buffer,
    diffuse_bind_group: grafx::BindGroup,
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: grafx::Buffer,
    camera_bind_group: grafx::BindGroup,
}

impl grafx::State for Game {
    fn new(ctx: &mut grafx::Context) -> Self {
        let shader = ctx.shader(std::fs::read_to_string("data/shader.wgsl").unwrap());
        let vertex_buffer = ctx.vertex_buffer(VERTICES);
        let index_buffer = ctx.index_buffer(INDICES);
        let mut pipeline_builder = grafx::PipelineBuilder::new(&shader).with_vertex::<Vertex>();
        let texture = ctx.load_texture_2d(std::fs::read("data/terrain.png").unwrap().as_ref());
        let sampler = ctx.sampler();
        let diffuse_bind_group = ctx.bind_group(
            grafx::BindGroupBuilder::new().with(&texture).with(&sampler),
            &mut pipeline_builder,
        );
        let camera = Camera::new();
        let camera_uniform = CameraUniform::new();
        let camera_buffer = ctx.uniform_buffer(&[camera_uniform]);
        let camera_bind_group = ctx.bind_group(
            grafx::BindGroupBuilder::new().with(&camera_buffer),
            &mut pipeline_builder,
        );

        let pipeline = ctx.pipeline(pipeline_builder);
        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            diffuse_bind_group,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
        }
    }

    fn resize(&mut self, ctx: &mut grafx::Context, width: f32, height: f32) {
        if width > 0.0 && height > 0.0 {
            self.camera_uniform.update_view_proj(&self.camera, width, height);
            ctx.write_buffer(&self.camera_buffer, 0, &[self.camera_uniform]);
        }
    }

    fn render(&self, frame: &mut grafx::Frame) {
        let mut pass = frame.pass(0.1, 0.2, 0.3, 1.0);
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        pass.set_bind_group(1, &self.camera_bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), grafx::IndexFormat::Uint16);
        pass.draw_indexed(0..6, 0, 0..1);
    }
}

fn main() {
    grafx::run::<Game>();
}
