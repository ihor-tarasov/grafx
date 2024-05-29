use crate::{PipelineBuilder, Sampler, Texture2D};

pub(crate) trait BindGroupEntry {
    fn visibility(&self) -> wgpu::ShaderStages;
    fn binding_type(&self) -> wgpu::BindingType;
    fn resource(&self) -> wgpu::BindingResource;
}

pub struct BindGroup(pub(crate) wgpu::BindGroup);

pub struct BindGroupBuilder(Vec<Box<dyn BindGroupEntry>>);

impl BindGroupBuilder {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub(crate) fn build(
        self,
        device: &wgpu::Device,
        pipeline_builder: &mut PipelineBuilder,
    ) -> BindGroup {
        let layout_entries = self
            .0
            .iter()
            .enumerate()
            .map(|(binding, entry)| wgpu::BindGroupLayoutEntry {
                binding: binding as u32,
                visibility: entry.visibility(),
                ty: entry.binding_type(),
                count: None,
            })
            .collect::<Box<_>>();
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &layout_entries,
        });
        let entries = self
            .0
            .iter()
            .enumerate()
            .map(|(binding, entry)| wgpu::BindGroupEntry {
                binding: binding as u32,
                resource: entry.resource(),
            })
            .collect::<Box<_>>();
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &entries,
        });
        pipeline_builder.with_bind_group_layout(layout);
        BindGroup(bind_group)
    }
}

pub trait BindGroupBuilderWith<T> {
    fn with(self, data: T) -> Self;
}

impl BindGroupBuilderWith<Texture2D> for BindGroupBuilder {
    fn with(mut self, data: Texture2D) -> Self {
        self.0.push(Box::new(data));
        self
    }
}

impl BindGroupBuilderWith<Sampler> for BindGroupBuilder {
    fn with(mut self, data: Sampler) -> Self {
        self.0.push(Box::new(data));
        self
    }
}
