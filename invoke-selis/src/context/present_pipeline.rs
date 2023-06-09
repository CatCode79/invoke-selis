use crate::{GlobalUniformBinding, Uniform};

pub(crate) struct PresentPipeline {
    pub(crate) pipeline: wgpu::RenderPipeline,
    _surface_format: wgpu::TextureFormat,
    sampler_bind_group: wgpu::BindGroup,
}

impl PresentPipeline {
    pub(crate) fn new(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
        module_desc: wgpu::ShaderModuleDescriptor<'_>,
    ) -> Self {
        let shader = device.create_shader_module(module_desc);
        let global_bind_group_layout = device.create_bind_group_layout(&Uniform::DESC);
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Present Texture BGL"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                }],
            });
        let sampler_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Present Sampler BGL"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                }],
            });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Screen Pass Layout"),
            bind_group_layouts: &[
                &global_bind_group_layout,
                &texture_bind_group_layout,
                &sampler_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Present Pipeline"),
            layout: Some(&layout),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[
                    Some(wgpu::ColorTargetState {
                        format: surface_format,
                        blend: None,
                        write_mask: Default::default(),
                    }),
                    Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        blend: None,
                        write_mask: Default::default(),
                    }),
                ],
            }),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                ..Default::default()
            },
            multiview: None,
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Present Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let sampler_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Present Sampler Bind Group"),
            layout: &sampler_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Sampler(&sampler),
            }],
        });

        Self {
            pipeline,
            _surface_format: surface_format,
            sampler_bind_group,
        }
    }
}

impl<'a> PresentPipeline {
    pub(crate) fn record<'pass>(
        &'a self,
        rpass: &mut wgpu::RenderPass<'pass>,
        uniform_bind_group: &'a GlobalUniformBinding,
        input_texture_binding: &'a wgpu::BindGroup,
    ) where
        'a: 'pass,
    {
        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, &uniform_bind_group.binding, &[]);
        rpass.set_bind_group(1, input_texture_binding, &[]);
        rpass.set_bind_group(2, &self.sampler_bind_group, &[]);
        rpass.draw(0..3, 0..1);
    }
}
