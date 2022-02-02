use std::f32::consts::{PI, TAU};

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

#[derive(Debug, Clone, Copy)]
pub struct QuadSphere {
    pub radius: f32,
    pub subdivisions: usize,
}

impl Default for QuadSphere {
    fn default() -> Self {
        Self {
            radius: 1.0,
            subdivisions: 1,
        }
    }
}
#[derive(Default)]
struct Surface {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

impl Surface {
    pub fn new(
        vertices: Vec<[f32; 3]>,
        normals: Vec<[f32; 3]>,
        uvs: Vec<[f32; 2]>,
        indices: Vec<u32>,
    ) -> Surface {
        Self {
            vertices,
            normals,
            uvs,
            indices,
        }
    }

    fn append(&mut self, mut surface: Surface) -> &mut Self {
        let offset = self.vertices.len() as u32;
        self.vertices.append(&mut surface.vertices);
        self.normals.append(&mut surface.normals);
        self.uvs.append(&mut surface.uvs);
        self.indices
            .append(&mut surface.indices.iter().map(|x| x + offset).collect());
        self
    }
}

impl From<QuadSphere> for Mesh {
    fn from(sphere: QuadSphere) -> Self {
        let mut surface = Surface::default();
        for face in [Vec3::X, Vec3::Y, Vec3::Z, -Vec3::X, -Vec3::Y, -Vec3::Z] {
            let x = create_surface(face, sphere);
            surface.append(x);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(surface.indices)));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, surface.vertices);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, surface.normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, surface.uvs);
        mesh
    }
}

fn create_surface(normal: Vec3, sphere: QuadSphere) -> Surface {
    let axis_a = Vec3::new(normal.y, normal.z, normal.x);
    let axis_b = normal.cross(axis_a);

    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(sphere.subdivisions.pow(2));
    let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(sphere.subdivisions.pow(2));
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(sphere.subdivisions.pow(2));
    let mut indices: Vec<u32> = Vec::with_capacity((sphere.subdivisions - 1).pow(2) * 6);

    let subdivisions = sphere.subdivisions as u32;
    for y in 0..sphere.subdivisions {
        for x in 0..sphere.subdivisions {
            let t = Vec2::new(x as f32, y as f32) / (sphere.subdivisions as f32 - 1.0);
            let point = normal + axis_a * (2.0 * t.x - 1.0) + axis_b * (2.0 * t.y - 1.0);
            let normal = map_cube_to_sphere(point);
            let vertex = normal * sphere.radius;
            let uv = map_sphere_to_uv(normal);

            normals.push(as_f32(normal));
            vertices.push(as_f32(vertex));
            uvs.push(uv);

            let vertex_index = (x + y * sphere.subdivisions) as u32;
            if x != sphere.subdivisions - 1 && y != sphere.subdivisions - 1 {
                indices.push(vertex_index);
                indices.push(vertex_index + subdivisions + 1);
                indices.push(vertex_index + subdivisions);
                indices.push(vertex_index);
                indices.push(vertex_index + 1);
                indices.push(vertex_index + subdivisions + 1);
            }
        }
    }

    Surface::new(vertices, normals, uvs, indices)
}

fn as_f32(p: Vec3) -> [f32; 3] {
    [p.x, p.y, p.z]
}

fn map_cube_to_sphere(point: Vec3) -> Vec3 {
    let x2 = point.x.powi(2);
    let y2 = point.y.powi(2);
    let z2 = point.z.powi(2);
    Vec3::new(
        point.x * (1.0 - y2 / 2.0 - z2 / 2.0 + y2 * z2 / 3.0).sqrt(),
        point.y * (1.0 - x2 / 2.0 - z2 / 2.0 + x2 * z2 / 3.0).sqrt(),
        point.z * (1.0 - x2 / 2.0 - y2 / 2.0 + x2 * y2 / 3.0).sqrt(),
    )
}

fn map_sphere_to_uv(point: Vec3) -> [f32; 2] {
    [
        point.x.atan2(point.z) / TAU + 0.5,
        0.5 - point.y.asin() / PI,
    ]
}
