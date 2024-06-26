use std::sync::Arc;
use winit::window::Window;
use wgpu::{Adapter, Features, SurfaceError};
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
    pub async fn new(window: Arc<Window>, buffer_size: usize) -> Self {
        let instance_descriptor = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(), ..Default::default()
        };
        let instance = wgpu::Instance::new(instance_descriptor);
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
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|f| {
                f.is_srgb()
            })
            .next()
            .unwrap_or(surface_capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

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

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let drawable = self.desc.surface.get_current_texture()?;

        let image_view_descriptor = wgpu::TextureViewDescriptor::default();

        let image_view = drawable.texture.create_view(&image_view_descriptor);

        self.desc.frame_buffer.write_to_inactive_buffer(vec![image_view]);

        self.desc.frame_buffer.swap_buffers();

        let command_encoder_descriptor = wgpu::CommandEncoderDescriptor {
            label: Some("Command Encoder"),
        };

        let mut encoder = self.desc.device.create_command_encoder(&command_encoder_descriptor);

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: self.desc.frame_buffer.get_active_buffer().lock().unwrap().as_ref().unwrap(),
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.75,
                    g: 0.5,
                    b: 0.25,
                    a: 1.0,
                }),
                store: wgpu::StoreOp::Store,
            },
        };

        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[color_attachment],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        };

        encoder.begin_render_pass(&render_pass_descriptor);

        self.desc.queue.submit(std::iter::once(encoder.finish()));

        drawable.present();
    }
}
