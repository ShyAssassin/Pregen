use glam::{Vec2, Vec3};
use bytemuck::{Pod, Zeroable};
use crate::gfx::VertexArrayObject;
use std::mem::{offset_of, size_of};


#[repr(C)]
#[derive(Debug)]
#[derive(Pod, Zeroable)]
#[derive(Copy, Clone, PartialEq)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub color: Vec3,
    pub uv: Vec2,
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, uv: Vec2, color: Option<Vec3>,) -> Self {
        let color = color.unwrap_or(rand::random::<Vec3>());
        Self {
            position,
            normal,
            color,
            uv,
        }
    }

    pub fn create_triangle(points: [Vec3; 3], normal: Vec3, uv: [Vec2; 3]) -> [Self; 3] {
        let color = rand::random::<Vec3>();
        return [
            Self::new(points[0], normal, uv[0], Some(color)),
            Self::new(points[1], normal, uv[1], Some(color)),
            Self::new(points[2], normal, uv[2], Some(color)),
        ]
    }

    pub fn from_points(points: [Vec3; 3], uv: [Vec2; 3]) -> [Self; 3] {
        // https://www.khronos.org/opengl/wiki/Calculating_a_Surface_Normal
        let u = points[1] - points[0];
        let v = points[2] - points[0];
        let normal = u.cross(v).normalize();
        return Self::create_triangle(points, normal, uv);
    }
}

impl VertexArrayObject for Vertex {
    const VERTEX_BUFFER_LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        step_mode: wgpu::VertexStepMode::Vertex,
        array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
        attributes: &[
            wgpu::VertexAttribute {
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
                offset: offset_of!(Vertex, position) as wgpu::BufferAddress,
            },
            wgpu::VertexAttribute {
                shader_location: 1,
                format: wgpu::VertexFormat::Float32x3,
                offset: offset_of!(Vertex, normal) as wgpu::BufferAddress,
            },
            wgpu::VertexAttribute {
                shader_location: 2,
                format: wgpu::VertexFormat::Float32x3,
                offset: offset_of!(Vertex, color) as wgpu::BufferAddress,
            },
            wgpu::VertexAttribute {
                shader_location: 3,
                format: wgpu::VertexFormat::Float32x2,
                offset: offset_of!(Vertex, uv) as wgpu::BufferAddress,
            }
        ]
    };
}
