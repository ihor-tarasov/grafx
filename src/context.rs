pub struct Context {
    device: wgpu::Device,
    queue: wgpu::Queue,
    format: wgpu::TextureFormat,
}

impl Context {
    pub(crate) async fn new(adapter: &wgpu::Adapter, format: wgpu::TextureFormat) -> Self {
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();
        Self {
            device,
            queue,
            format,
        }
    }

    pub(crate) fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub(crate) fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub(crate) fn format(&self) -> wgpu::TextureFormat {
        self.format
    }
}
