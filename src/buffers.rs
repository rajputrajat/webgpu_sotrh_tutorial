use bytemuck;
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
        position: [-0.0868241, 0.49240386, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.49513406, 0.069585847, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        color: [0.0, 0.0, 1.0],
    },
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        color: [0.0, 1.0, 0.0],
    },
];

pub const INCICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];
