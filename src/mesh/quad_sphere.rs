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

impl From<QuadSphere> for Mesh {
    fn from(sphere: QuadSphere) -> Self {
        let (mut vertices, mut normals, mut uvs, mut indices) =
            create_face(Vec3::X, sphere.radius, sphere.subdivisions, 0);

        let (mut y_vertices, mut y_normals, mut y_uvs, mut y_indices) =
            create_face(Vec3::Y, sphere.radius, sphere.subdivisions, vertices.len());

        vertices.append(&mut y_vertices);
        normals.append(&mut y_normals);
        uvs.append(&mut y_uvs);
        indices.append(&mut y_indices);

        let (mut y_vertices, mut y_normals, mut y_uvs, mut y_indices) =
            create_face(Vec3::Z, sphere.radius, sphere.subdivisions, vertices.len());

        vertices.append(&mut y_vertices);
        normals.append(&mut y_normals);
        uvs.append(&mut y_uvs);
        indices.append(&mut y_indices);

        let (mut y_vertices, mut y_normals, mut y_uvs, mut y_indices) =
            create_face(-Vec3::X, sphere.radius, sphere.subdivisions, vertices.len());

        vertices.append(&mut y_vertices);
        normals.append(&mut y_normals);
        uvs.append(&mut y_uvs);
        indices.append(&mut y_indices);

        let (mut y_vertices, mut y_normals, mut y_uvs, mut y_indices) =
            create_face(-Vec3::Y, sphere.radius, sphere.subdivisions, vertices.len());

        vertices.append(&mut y_vertices);
        normals.append(&mut y_normals);
        uvs.append(&mut y_uvs);
        indices.append(&mut y_indices);

        let (mut y_vertices, mut y_normals, mut y_uvs, mut y_indices) =
            create_face(-Vec3::Z, sphere.radius, sphere.subdivisions, vertices.len());

        vertices.append(&mut y_vertices);
        normals.append(&mut y_normals);
        uvs.append(&mut y_uvs);
        indices.append(&mut y_indices);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}

fn create_face(
    normal: Vec3,
    radius: f32,
    subdivisions: usize,
    offset: usize,
) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<u32>) {
    let length_inv = 1. / radius;

    let axis_a = Vec3::new(normal.y, normal.z, normal.x);
    let axis_b = normal.cross(axis_a);

    let mut vertices = vec![[0.0, 0.0, 0.0]; subdivisions * subdivisions];
    let mut normals = vec![[0.0, 0.0, 0.0]; subdivisions * subdivisions];
    let mut uvs = vec![[0.0, 0.0]; subdivisions * subdivisions];
    let mut indices = vec![0; (subdivisions - 1) * (subdivisions - 1) * 6];

    let mut indices_index = 0;

    for y in 0..subdivisions {
        for x in 0..subdivisions {
            let vertex_index = x + y * subdivisions;
            let t = Vec2::new(x as f32, y as f32) / (subdivisions as f32 - 1.0);
            let point = normal + axis_a * (2.0 * t.x - 1.0) + axis_b * (2.0 * t.y - 1.0);

            // let point_n = point.clone() / (point.x * point.x + point.y * point.y + point.z * point.z).sqrt();
            let point_n = map_cube_to_sphere(point);
            let point_v = point_n * radius;

            normals[vertex_index] = [point_n.x, point_n.y, point_n.z];
            vertices[vertex_index] = [point_v.x, point_v.y, point_v.z];

            // vertices[vertex_index] = [point.x, point.y, point.z];
            // normals[vertex_index] = [
            //     point.x * length_inv,
            //     point.y * length_inv,
            //     point.z * length_inv,
            // ];
            uvs[vertex_index] = [
                (y as f32) / subdivisions as f32,
                (x as f32) / subdivisions as f32,
            ];
            if x != subdivisions - 1 && y != subdivisions - 1 {
                indices[indices_index + 0] = (offset + vertex_index) as u32;
                indices[indices_index + 1] = (offset + vertex_index + subdivisions + 1) as u32;
                indices[indices_index + 2] = (offset + vertex_index + subdivisions) as u32;
                indices[indices_index + 3] = (offset + vertex_index) as u32;
                indices[indices_index + 4] = (offset + vertex_index + 1) as u32;
                indices[indices_index + 5] = (offset + vertex_index + subdivisions + 1) as u32;
                indices_index += 6;
            }
        }
    }

    (vertices, normals, uvs, indices)
}

fn map_cube_to_sphere(point: Vec3) -> Vec3 {
    let x2 = point.x.powi(2);
    let y2 = point.y.powi(2);
    let z2 = point.z.powi(2);
    return Vec3::new(
        point.x * (1.0 - y2 / 2.0 - z2 / 2.0 + y2 * z2 / 3.0).sqrt(),
        point.y * (1.0 - x2 / 2.0 - z2 / 2.0 + x2 * z2 / 3.0).sqrt(),
        point.z * (1.0 - x2 / 2.0 - y2 / 2.0 + x2 * y2 / 3.0).sqrt(),
    );
}
