use std::borrow::Cow;

pub struct PipelineBuilder {
    shader_filename: String,
    vertex_entry: String,
    fragment_entry: String,
    pixel_format: wgpu::TextureFormat,
}

impl PipelineBuilder {
    pub fn new(shader_filename: &str) -> Self {
        PipelineBuilder {
            shader_filename: shader_filename.to_string(),
            vertex_entry: "main".to_string(),
            fragment_entry: "main".to_string(),
            pixel_format: wgpu::TextureFormat::Bgra8UnormSrgb,
        }
    }

    pub fn with_vertex_entry(mut self, entry: &str) -> Self {
        self.vertex_entry = entry.to_string();
        self
    }

    pub fn with_fragment_entry(mut self, entry: &str) -> Self {
        self.fragment_entry = entry.to_string();
        self
    }

    pub fn with_pixel_format(mut self, format: wgpu::TextureFormat) -> Self {
        self.pixel_format = format;
        self
    }

    pub fn build_pipeline(&self, device: &wgpu::Device) -> wgpu::RenderPipeline {
        let mut filepath = std::env::current_dir().unwrap();
        filepath.push("shaders");
        filepath.push(&self.shader_filename);
        let filepath = filepath.into_os_string().into_string().unwrap();
        let source_code: Cow<'_, str> = std::fs::read_to_string(filepath).unwrap().into();

        let shader = wgpu::ShaderModuleDescriptor {
            label: Some(&self.shader_filename),
            source: wgpu::ShaderSource::Wgsl(source_code),
        };

        let shader_module = device.create_shader_module(shader);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&self.shader_filename),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_targets = &[Some(wgpu::ColorTargetState {
            format: self.pixel_format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let render_pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: Some(&self.shader_filename),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: &self.vertex_entry,
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: &self.fragment_entry,
                compilation_options: Default::default(),
                targets: render_targets,
            }),
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
            multiview: None,
        };

        device.create_render_pipeline(&render_pipeline_descriptor)
    }
 }