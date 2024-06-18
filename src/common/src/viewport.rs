use std::sync::Arc;
use winit::window::Window;
use wgpu::{Adapter, Features};
use crate::dbl_buffer;

#[derive(Debug)]
pub struct ViewportDesc {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    frame_buffer: dbl_buffer::DoubleBuffer<wgpu::TextureView>,
    adapter: Adapter,
}

pub struct Viewport {
    desc: ViewportDesc,
    config: wgpu::SurfaceConfiguration,
}

impl ViewportDesc {
    pub async fn new(window: Arc<Window>, instance: &wgpu::Instance, buffer_size: usize) -> Self {
        let surface = instance.create_surface(window.clone());

        if !surface.is_err() {
            panic!("Surface is not supported");
        }

        let surface = surface
            .expect("Error creating surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Error requesting adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device"),
                    required_features: Features::default(),
                    required_limits: Default::default(),
                },
                None,
            )
            .await
            .expect("Error requesting device");

        let size = window.inner_size();
        let config = surface.get_default_config(&adapter, size.width, size.height)
            .expect("Error getting default config");

        surface.configure(&device, &config);

        let frame_buffer = dbl_buffer::DoubleBuffer::new(buffer_size);

        Self {
            window,
            surface,
            device,
            queue,
            size,
            frame_buffer,
            adapter
        }
    }

    pub fn build(self, device: &wgpu::Device) -> Viewport {
        let size = self.window.inner_size();
        let config = self
            .surface
            .get_default_config(&self.adapter, size.width, size.height)
            .unwrap();
        self.surface.configure(device, &config);
        Viewport { desc: self, config }
    }
}

impl Viewport {

    pub fn resize(&mut self, device: &wgpu::Device, size: winit::dpi::PhysicalSize<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;
        self.desc.surface.configure(device, &self.config);
    }

    pub fn get_current_texture(&mut self) -> wgpu::SurfaceTexture {
        self.desc
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture")
    }
}
