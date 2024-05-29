use std::ops::Range;

use glam::*;

use crate::{buffer, pipeline, BindGroup, DynamicOffset};

pub struct Pass<'a>(wgpu::RenderPass<'a>);

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

    pub fn set_pipeline(&mut self, pipeline: &'a pipeline::Pipeline) {
        self.0.set_pipeline(&pipeline.0);
    }

    pub fn set_vertex_buffer(&mut self, slot: u32, slice: buffer::BufferSlice<'a>) {
        self.0.set_vertex_buffer(slot, slice.0);
    }

    pub fn set_index_buffer(
        &mut self,
        slice: buffer::BufferSlice<'a>,
        format: buffer::IndexFormat,
    ) {
        self.0.set_index_buffer(slice.0, format);
    }

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.0.draw(vertices, instances);
    }

    pub fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.0.draw_indexed(indices, base_vertex, instances);
    }

    pub fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: &'a BindGroup,
        offsets: &[DynamicOffset],
    ) {
        self.0.set_bind_group(index, &bind_group.0, offsets);
    }
}
