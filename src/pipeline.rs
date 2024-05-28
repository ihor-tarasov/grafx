use crate::vertex;

pub struct Shader(pub(crate) wgpu::ShaderModule);

impl Shader {
    pub(crate) fn new(device: &wgpu::Device, src: String) -> Self {
        Self(device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(src.into()),
        }))
    }
}

pub struct PipelineBuilder<'a> {
    buffers: Vec<vertex::Layout>,
    shader: &'a Shader,
}

impl<'a> PipelineBuilder<'a> {
    pub fn new(shader: &'a Shader) -> Self {
        Self {
            buffers: Vec::new(),
            shader,
        }
    }

    pub fn with_vertex<T: vertex::Vertex>(mut self) -> Self {
        self.buffers.push(T::layout());
        self
    }
}

pub struct Pipeline(pub(crate) wgpu::RenderPipeline);

impl Pipeline {
    pub(crate) fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        builder: PipelineBuilder,
    ) -> Self {
        let buffsers = builder
            .buffers
            .iter()
            .map(|layout| layout.as_vertex_buffer_layout())
            .collect::<Box<_>>();
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        Self(
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &builder.shader.0,
                    entry_point: "vs_main",
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    buffers: &buffsers,
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &builder.shader.0,
                    entry_point: "fs_main",
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview: None,
            }),
        )
    }
}
