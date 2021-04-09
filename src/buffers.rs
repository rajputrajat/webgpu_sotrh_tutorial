use wgpu::{self, BufferAddress, InputStepMode, VertexAttribute, VertexBufferLayout};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    pub fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: InputStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float3,
                },
            ],
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.086_824_1, 0.492_403_86, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.495_134_06, 0.069_585_845, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [-0.219_185_49, -0.449_397_06, 0.0],
        color: [0.0, 0.0, 1.0],
    },
    Vertex {
        position: [0.359_669_98, -0.347_329_1, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.441_473_72, 0.234_735_9, 0.0],
        color: [0.0, 1.0, 0.0],
    },
];

pub const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];
