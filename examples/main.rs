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
    direction: Vec3,
    speed: f32,
    yaw: f32,
    pitch: f32,
    projection: Mat4,
    view: Mat4,
    locked: bool,
}

impl Camera {
    fn new(ctx: &grafx::Context, speed: f32) -> Self {
        ctx.set_cursor_position(ctx.size() / 2.0);
        Self {
            position: vec3(0.0, 1.0, 2.0),
            direction: vec3(0.0, 0.0, -1.0),
            projection: Mat4::IDENTITY,
            view: Mat4::IDENTITY,
            speed,
            yaw: -90.0f32.to_radians(),
            pitch: 0.0,
            locked: true,
        }
    }

    fn update_projection(&mut self, size: Vec2) {
        self.projection = Mat4::perspective_rh(76f32.to_radians(), size.x / size.y, 0.1, 1000.0);
    }

    fn uniform(&self) -> CameraUniform {
        CameraUniform {
            view_proj: (self.projection * self.view).to_cols_array_2d(),
        }
    }

    fn key(&mut self, code: grafx::KeyCode, pressed: bool) {
        match code {
            grafx::KeyCode::Escape => {
                if pressed {
                    self.locked = !self.locked;
                }
            }
            _ => {}
        }
    }

    fn cursor(&mut self, ctx: &grafx::Context, pos: Vec2) {
        if self.locked {
            let sensitivity = 0.002;
            let center = ctx.size() / 2.0;
            let cursor_delta = pos - center;
            ctx.set_cursor_position(center);
            self.yaw += cursor_delta.x * sensitivity;
            self.pitch -= cursor_delta.y * sensitivity;

            if self.pitch > 89.0f32.to_radians() {
                self.pitch = 89.0f32.to_radians();
            }
            if self.pitch < -89.0f32.to_radians() {
                self.pitch = -89.0f32.to_radians();
            }

            self.direction = vec3(
                self.yaw.cos() * self.pitch.cos(),
                self.pitch.sin(),
                self.yaw.sin() * self.pitch.cos(),
            );
        }
    }

    fn update_view(&mut self, ctx: &grafx::Context, delta: f32) {
        if ctx.key(grafx::KeyCode::KeyW) {
            self.position += self.direction * self.speed * delta;
        }
        if ctx.key(grafx::KeyCode::KeyS) {
            self.position -= self.direction * self.speed * delta;
        }
        if ctx.key(grafx::KeyCode::KeyA) {
            self.position -= self.direction.cross(Vec3::Y).normalize() * self.speed * delta;
        }
        if ctx.key(grafx::KeyCode::KeyD) {
            self.position += self.direction.cross(Vec3::Y).normalize() * self.speed * delta;
        }
        if ctx.key(grafx::KeyCode::Space) {
            self.position += Vec3::Y * self.speed * delta;
        }
        if ctx.key(grafx::KeyCode::ShiftLeft) {
            self.position -= Vec3::Y * self.speed * delta;
        }
        self.view = Mat4::look_to_rh(self.position, self.direction, Vec3::Y);
    }
}

grafx::impl_uniform! {
    struct CameraUniform {
        view_proj: [[f32; 4]; 4],
    }
}

struct Game {
    pipeline: grafx::Pipeline,
    vertex_buffer: grafx::VertexBuffer,
    index_buffer: grafx::IndexBufferU16,
    diffuse_bind_group: grafx::BindGroup,
    camera: Camera,
    camera_buffer: grafx::UniformBuffer,
    camera_bind_group: grafx::BindGroup,
}

impl grafx::State for Game {
    fn new(ctx: &grafx::Context) -> Self {
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
        let mut camera = Camera::new(ctx, 1.0);
        camera.update_projection(ctx.size());
        let camera_buffer = grafx::UniformBuffer::new(ctx, &[camera.uniform()]);
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
            camera_buffer,
            camera_bind_group,
        }
    }

    fn resize(&mut self, ctx: &grafx::Context, size: Vec2) {
        if size.x > 0.0 && size.y > 0.0 {
            self.camera.update_projection(size);
            self.camera_buffer.write(ctx, 0, &[self.camera.uniform()]);
        }
    }

    fn key(&mut self, _ctx: &grafx::Context, code: grafx::KeyCode, pressed: bool) {
        self.camera.key(code, pressed);
    }

    fn cursor(&mut self, ctx: &grafx::Context, pos: Vec2) {
        self.camera.cursor(ctx, pos);
    }

    fn update(&mut self, ctx: &grafx::Context, delta: Duration) {
        self.camera.update_view(ctx, delta.as_secs_f32());
        self.camera_buffer.write(ctx, 0, &[self.camera.uniform()]);
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
