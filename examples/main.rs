use std::time::Duration;

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
    projection: Mat4,
    speed: f32,
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
}

impl Camera {
    fn new(speed: f32) -> Self {
        Self {
            position: vec3(0.0, 1.0, 2.0),
            target: vec3(0.0, 0.0, 0.0),
            projection: Mat4::IDENTITY,
            speed,
            forward: false,
            backward: false,
            left: false,
            right: false,
        }
    }

    fn update_projection(&mut self, width: f32, height: f32) {
        self.projection = Mat4::perspective_rh(76f32.to_radians(), width / height, 0.1, 1000.0);
    }

    fn matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.position, self.target, Vec3::Y);
        self.projection * view
    }

    fn key(&mut self, code: grafx::KeyCode, pressed: bool) {
        match code {
            grafx::KeyCode::KeyW | grafx::KeyCode::ArrowUp => self.forward = pressed,
            grafx::KeyCode::KeyA | grafx::KeyCode::ArrowLeft => self.left = pressed,
            grafx::KeyCode::KeyS | grafx::KeyCode::ArrowDown => self.backward = pressed,
            grafx::KeyCode::KeyD | grafx::KeyCode::ArrowRight => self.right = pressed,
            _ => {}
        }
    }

    fn update_view(&mut self, delta: f32) {
        let forward = self.target - self.position;
        let forward_norm = forward.normalize();
        let forward_mag = forward.length();

        // Prevents glitching when the camera gets too close to the
        // center of the scene.
        if self.forward && forward_mag > self.speed {
            self.position += forward_norm * self.speed * delta;
        }
        if self.backward {
            self.position -= forward_norm * self.speed * delta;
        }

        let right = forward_norm.cross(Vec3::Y);

        // Redo radius calc in case the forward/backward is pressed.
        let forward = self.target - self.position;
        let forward_mag = forward.length();

        if self.right {
            // Rescale the distance between the target and the eye so
            // that it doesn't change. The eye, therefore, still
            // lies on the circle made by the target and eye.
            self.position =
                self.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.left {
            self.position =
                self.target - (forward - right * self.speed).normalize() * forward_mag;
        }
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

    fn update(&mut self, camera: &Camera) {
        self.view_proj = camera.matrix().to_cols_array_2d();
    }
}

struct Game {
    pipeline: grafx::Pipeline,
    vertex_buffer: grafx::VertexBuffer,
    index_buffer: grafx::IndexBufferU16,
    diffuse_bind_group: grafx::BindGroup,
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: grafx::UniformBuffer,
    camera_bind_group: grafx::BindGroup,
}

impl grafx::State for Game {
    fn new(ctx: &mut grafx::Context) -> Self {
        let shader = grafx::Shader::new(ctx, std::fs::read_to_string("data/shader.wgsl").unwrap());
        let vertex_buffer = grafx::VertexBuffer::new(ctx, VERTICES);
        let index_buffer = grafx::IndexBufferU16::new(ctx, INDICES);
        let mut pipeline_builder = grafx::PipelineBuilder::new(&shader).with_vertex::<Vertex>();
        let texture =
            grafx::Texture2D::new(ctx, std::fs::read("data/terrain.png").unwrap().as_ref());
        let sampler = grafx::Sampler::new(ctx);
        let diffuse_bind_group = grafx::BindGroupBuilder::new()
            .with(&texture)
            .with(&sampler)
            .build(ctx, &mut pipeline_builder);
        let camera = Camera::new(0.2);
        let camera_uniform = CameraUniform::new();
        let camera_buffer = grafx::UniformBuffer::new(ctx, &[camera_uniform]);
        let camera_bind_group = grafx::BindGroupBuilder::new()
            .with(&camera_buffer)
            .build(ctx, &mut pipeline_builder);
        let pipeline = pipeline_builder.build(ctx);
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
            self.camera.update_projection(width, height);
            self.camera_uniform.update(&self.camera);
            self.camera_buffer.write(ctx, 0, &[self.camera_uniform]);
        }
    }

    fn key(&mut self, _ctx: &mut grafx::Context, code: grafx::KeyCode, pressed: bool) {
        self.camera.key(code, pressed);
    }

    fn update(&mut self, delta: Duration, ctx: &mut grafx::Context) {
        self.camera.update_view(delta.as_secs_f32());
        self.camera_uniform.update(&self.camera);
        self.camera_buffer.write(ctx, 0, &[self.camera_uniform]);
    }

    fn render(&self, frame: &mut grafx::Frame) {
        let mut pass = frame.pass(0.1, 0.2, 0.3, 1.0);
        self.pipeline.attach(&mut pass);
        self.diffuse_bind_group.attach(&mut pass, 0, &[]);
        self.camera_bind_group.attach(&mut pass, 1, &[]);
        self.vertex_buffer.attach(&mut pass, 0, ..);
        self.index_buffer.attach(&mut pass, ..);
        pass.draw_indexed(0..6, 0, 0..1);
    }
}

fn main() {
    grafx::run::<Game>();
}
