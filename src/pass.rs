use std::ops::Range;

use glam::*;

pub struct Pass<'a>(pub(crate) wgpu::RenderPass<'a>);

impl<'a> Pass<'a> {
    pub(crate) fn new(
        encoder: &'a mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) -> Self {
        Self(encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: r as f64,
                        g: g as f64,
                        b: b as f64,
                        a: a as f64,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        }))
    }

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.0.draw(vertices, instances);
    }

    pub fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.0.draw_indexed(indices, base_vertex, instances);
    }
}
