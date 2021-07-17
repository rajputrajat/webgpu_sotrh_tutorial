use crate::{buffers, swapchain};
use image::{self, GenericImageView};
use std::num::NonZeroU32;
use wgpu::{
    include_spirv,
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsage, ColorTargetState, ColorWrite, Device, Face, FragmentState, FrontFace,
    MultisampleState, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology,
    Queue, RenderPipeline, RenderPipelineDescriptor, SwapChainDescriptor, VertexState,
};

pub(crate) struct BufferRelatedData {
    pub(crate) vertex_buffer: Buffer,
    pub(crate) index_buffer: Buffer,
    pub(crate) num_indices: u32,
}

pub(crate) struct SpecificRender {
    pub(crate) render_pipeline: RenderPipeline,
    pub(crate) buffer_related: Option<BufferRelatedData>,
}

impl swapchain::State {
    pub(crate) fn create_specific_render_pipelines(
        device: &Device,
        sc_desc: &SwapChainDescriptor,
    ) -> Vec<SpecificRender> {
        let primitive = PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: Some(Face::Back),
            polygon_mode: PolygonMode::Fill,
            clamp_depth: false,
            conservative: false,
        };
        let multisample = MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        };
        let simple_specific_render;
        {
            let vs_module = device.create_shader_module(&include_spirv!("shader.vert.spv"));
            let fs_module = device.create_shader_module(&include_spirv!("shader.frag.spv"));
            // create pipeline layout
            let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("render pipeline layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
            // create render pipeline
            let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("render pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: VertexState {
                    module: &vs_module,
                    entry_point: "main",
                    buffers: &[],
                },
                fragment: Some(FragmentState {
                    module: &fs_module,
                    entry_point: "main",
                    targets: &[ColorTargetState {
                        format: sc_desc.format,
                        blend: None,
                        write_mask: ColorWrite::ALL,
                    }],
                }),
                primitive: primitive,
                depth_stencil: None,
                multisample: multisample,
            });
            simple_specific_render = SpecificRender {
                render_pipeline,
                buffer_related: None,
            };
        }
        let challenge_specific_render;
        {
            let vs_module = device.create_shader_module(&include_spirv!("challenge.vert.spv"));
            let fs_module = device.create_shader_module(&include_spirv!("challenge.frag.spv"));
            // create pipeline layout
            let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("render pipeline layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
            // create render pipeline
            let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("challenge render pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: VertexState {
                    module: &vs_module,
                    entry_point: "main",
                    buffers: &[buffers::Vertex::desc()],
                },
                fragment: Some(FragmentState {
                    module: &fs_module,
                    entry_point: "main",
                    targets: &[ColorTargetState {
                        format: sc_desc.format,
                        blend: None,
                        write_mask: ColorWrite::ALL,
                    }],
                }),
                primitive: primitive.clone(),
                depth_stencil: None,
                multisample: multisample.clone(),
            });
            let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: Some("vertex buffer"),
                contents: bytemuck::cast_slice(buffers::PENTAGON_VERTICES),
                usage: BufferUsage::VERTEX,
            });
            let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: Some("index buffer"),
                contents: bytemuck::cast_slice(buffers::PENTAGON_INDICES),
                usage: BufferUsage::INDEX,
            });
            let num_indices = buffers::PENTAGON_INDICES.len() as u32;
            challenge_specific_render = SpecificRender {
                render_pipeline,
                buffer_related: Some(BufferRelatedData {
                    vertex_buffer,
                    index_buffer,
                    num_indices,
                }),
            };
        }
        let challenge2_specific_render;
        {
            let vs_module = device.create_shader_module(&include_spirv!("challenge.vert.spv"));
            let fs_module = device.create_shader_module(&include_spirv!("challenge.frag.spv"));
            // create pipeline layout
            let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("render pipeline layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
            // create render pipeline
            let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("challenge 2 hexagon ender pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: VertexState {
                    module: &vs_module,
                    entry_point: "main",
                    buffers: &[buffers::Vertex::desc()],
                },
                fragment: Some(FragmentState {
                    module: &fs_module,
                    entry_point: "main",
                    targets: &[ColorTargetState {
                        format: sc_desc.format,
                        blend: None,
                        write_mask: ColorWrite::ALL,
                    }],
                }),
                primitive,
                depth_stencil: None,
                multisample,
            });
            let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: Some("vertex buffer"),
                contents: bytemuck::cast_slice(buffers::HEXAGON_VERTICES),
                usage: BufferUsage::VERTEX,
            });
            let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: Some("index buffer"),
                contents: bytemuck::cast_slice(buffers::HEXAGON_INDICES),
                usage: BufferUsage::INDEX,
            });
            let num_indices = buffers::HEXAGON_INDICES.len() as u32;
            challenge2_specific_render = SpecificRender {
                render_pipeline,
                buffer_related: Some(BufferRelatedData {
                    vertex_buffer,
                    index_buffer,
                    num_indices,
                }),
            };
        }
        vec![
            simple_specific_render,
            challenge_specific_render,
            challenge2_specific_render,
        ]
    }

    pub(crate) fn create_texture(device: &Device, queue: &Queue) {
        let diffuse_bytes = include_bytes!("neutron.jpg");
        let diffuse_img = image::load_from_memory(diffuse_bytes).unwrap();
        let diffuse_rgba = diffuse_img.as_rgba8().unwrap();
        let dimension = diffuse_img.dimensions();
        let tex_size = wgpu::Extent3d {
            width: dimension.0,
            height: dimension.1,
            depth_or_array_layers: 1,
        };
        let diffuse_tex = device.create_texture(&wgpu::TextureDescriptor {
            size: tex_size,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            label: Some("diffuse_texture"),
        });
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &diffuse_tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            diffuse_rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * dimension.0),
                rows_per_image: NonZeroU32::new(dimension.1),
            },
            tex_size,
        );
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("temp buffer for texture"),
            contents: &diffuse_rgba,
            usage: wgpu::BufferUsage::COPY_DST,
        });
    }
}
