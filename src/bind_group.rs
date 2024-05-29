use crate::{Context, Pass, PipelineBuilder, Sampler, Texture2D, UniformBuffer};

pub(crate) trait BindGroupEntry {
    fn visibility(&self) -> wgpu::ShaderStages;
    fn binding_type(&self) -> wgpu::BindingType;
    fn resource(&self) -> wgpu::BindingResource;
}

pub struct BindGroup(wgpu::BindGroup);

impl BindGroup {
    pub fn attach<'a>(&'a self, pass: &mut Pass<'a>, index: u32, offsets: &[u32]) {
        pass.0.set_bind_group(index, &self.0, offsets);
    }
}

pub struct BindGroupBuilder<'a>(Vec<&'a dyn BindGroupEntry>);

impl<'a> BindGroupBuilder<'a> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn build(self, ctx: &Context, pipeline_builder: &mut PipelineBuilder) -> BindGroup {
        let layout_entries = self
            .0
            .iter()
            .copied()
            .enumerate()
            .map(|(binding, entry)| wgpu::BindGroupLayoutEntry {
                binding: binding as u32,
                visibility: entry.visibility(),
                ty: entry.binding_type(),
                count: None,
            })
            .collect::<Box<_>>();
        let layout = ctx
            .device()
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &layout_entries,
            });
        let entries = self
            .0
            .iter()
            .copied()
            .enumerate()
            .map(|(binding, entry)| wgpu::BindGroupEntry {
                binding: binding as u32,
                resource: entry.resource(),
            })
            .collect::<Box<_>>();
        let bind_group = ctx.device().create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &entries,
        });
        pipeline_builder.with_bind_group_layout(layout);
        BindGroup(bind_group)
    }
}

pub trait BindGroupBuilderWith<'a, T> {
    fn with(self, data: &'a T) -> Self;
}

impl<'a> BindGroupBuilderWith<'a, Texture2D> for BindGroupBuilder<'a> {
    fn with(mut self, data: &'a Texture2D) -> Self {
        self.0.push(data);
        self
    }
}

impl<'a> BindGroupBuilderWith<'a, Sampler> for BindGroupBuilder<'a> {
    fn with(mut self, data: &'a Sampler) -> Self {
        self.0.push(data);
        self
    }
}

impl<'a> BindGroupBuilderWith<'a, UniformBuffer> for BindGroupBuilder<'a> {
    fn with(mut self, data: &'a UniformBuffer) -> Self {
        self.0.push(data);
        self
    }
}
