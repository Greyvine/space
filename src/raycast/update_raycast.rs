use std::{
    collections::BTreeMap,
    f32::EPSILON,
    sync::{Arc, Mutex},
};

use bevy::{
    core::FloatOrd,
    math::Vec3A,
    prelude::*,
    render::mesh::{Indices, VertexAttributeValues},
    tasks::ComputeTaskPool,
};

use super::{
    primitives::{Backfaces, Intersection, IntoUsize, RayHit, Triangle, TriangleTrait},
    ray::Ray3d,
    RayCastMesh, RayCastSource,
};

#[allow(clippy::type_complexity)]
pub fn update_raycast<T: 'static + Send + Sync>(
    meshes: Res<Assets<Mesh>>,
    task_pool: Res<ComputeTaskPool>,
    mut source_query: Query<&mut RayCastSource<T>>,
    mesh_query: Query<(&Handle<Mesh>, &GlobalTransform, Entity), With<RayCastMesh<T>>>,
) {
    for mut source in source_query.iter_mut() {
        if let Some(ray) = source.ray {
            source.intersections.clear();
            let picks = Arc::new(Mutex::new(BTreeMap::new()));
            mesh_query.par_for_each(&task_pool, 32, |(mesh_handle, transform, entity)| {
                meshes
                    .get(mesh_handle)
                    .and_then(|x| compute_intersection(x, &transform.compute_matrix(), &ray))
                    .and_then(|intersection| {
                        println!("{:?}", intersection);
                        picks
                            .lock()
                            .unwrap()
                            .insert(FloatOrd(intersection.distance()), (entity, intersection))
                    });
            });
            let picks = Arc::try_unwrap(picks).unwrap().into_inner().unwrap();
            source.intersections = picks.into_values().collect();
        }
    }
}

pub fn compute_intersection(
    mesh: &Mesh,
    mesh_to_world: &Mat4,
    ray: &Ray3d,
) -> Option<Intersection> {
    let positions = match mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .expect("Mesh does not contain vertex positions")
    {
        VertexAttributeValues::Float32x3(positions) => positions,
        _ => panic!("Unexpected types in {}", Mesh::ATTRIBUTE_POSITION),
    };

    let normals: Option<&[[f32; 3]]> =
        if let Some(normal_values) = mesh.attribute(Mesh::ATTRIBUTE_NORMAL) {
            match &normal_values {
                VertexAttributeValues::Float32x3(normals) => Some(normals),
                _ => None,
            }
        } else {
            None
        };

    if let Some(indices) = &mesh.indices() {
        // Iterate over the list of pick rays that belong to the same group as this mesh
        match indices {
            Indices::U16(vertex_indices) => {
                _compute_intersection(mesh_to_world, positions, normals, ray, Some(vertex_indices))
            }
            Indices::U32(vertex_indices) => {
                _compute_intersection(mesh_to_world, positions, normals, ray, Some(vertex_indices))
            }
        }
    } else {
        None
    }
}

fn _compute_intersection(
    mesh_to_world: &Mat4,
    vertex_positions: &[[f32; 3]],
    vertex_normals: Option<&[[f32; 3]]>,
    pick_ray: &Ray3d,
    indices: Option<&Vec<impl IntoUsize>>,
) -> Option<Intersection> {
    let mut min_pick_distance = f32::MAX;
    let mut pick_intersection = None;

    let world_to_mesh = mesh_to_world.inverse();

    let mesh_space_ray = Ray3d::new(
        world_to_mesh.transform_point3(pick_ray.origin.into()),
        world_to_mesh.transform_vector3(pick_ray.direction.into()),
    );

    if let Some(indices) = indices {
        // Make sure this chunk has 3 vertices to avoid a panic.
        if indices.len() % 3 != 0 {
            warn!("Index list not a multiple of 3");
            return None;
        }
        // Now that we're in the vector of vertex indices, we want to look at the vertex
        // positions for each triangle, so we'll take indices in chunks of three, where each
        // chunk of three indices are references to the three vertices of a triangle.
        for index in indices.chunks(3) {
            let tri_vertex_positions = [
                Vec3A::from(vertex_positions[index[0].into_usize()]),
                Vec3A::from(vertex_positions[index[1].into_usize()]),
                Vec3A::from(vertex_positions[index[2].into_usize()]),
            ];
            let tri_normals = vertex_normals.map(|normals| {
                [
                    Vec3A::from(normals[index[0].into_usize()]),
                    Vec3A::from(normals[index[1].into_usize()]),
                    Vec3A::from(normals[index[2].into_usize()]),
                ]
            });
            let intersection = triangle_intersection(
                tri_vertex_positions,
                tri_normals,
                min_pick_distance,
                mesh_space_ray,
            );
            if let Some(i) = intersection {
                pick_intersection = Some(Intersection::new(
                    mesh_to_world.transform_point3(i.position),
                    mesh_to_world.transform_vector3(i.normal),
                    mesh_to_world
                        .transform_vector3(mesh_space_ray.direction() * i.distance)
                        .length(),
                    i.triangle.map(|tri| {
                        Triangle::from([
                            mesh_to_world.transform_point3a(tri.v0),
                            mesh_to_world.transform_point3a(tri.v1),
                            mesh_to_world.transform_point3a(tri.v2),
                        ])
                    }),
                ));
                min_pick_distance = i.distance();
            }
        }
    }

    pick_intersection
}

fn triangle_intersection(
    tri_vertices: [Vec3A; 3],
    tri_normals: Option<[Vec3A; 3]>,
    max_distance: f32,
    ray: Ray3d,
) -> Option<Intersection> {
    if tri_vertices
        .iter()
        .any(|&vertex| (vertex - ray.origin).length_squared() < max_distance.powi(2))
    {
        // Run the raycast on the ray and triangle
        if let Some(ray_hit) = ray_triangle_intersection(&ray, &tri_vertices, Backfaces::default())
        {
            let distance = *ray_hit.distance();
            if distance > 0.0 && distance < max_distance {
                let position = ray.position(distance);
                let normal = if let Some(normals) = tri_normals {
                    let u = ray_hit.uv_coords().0;
                    let v = ray_hit.uv_coords().1;
                    let w = 1.0 - u - v;
                    normals[1] * u + normals[2] * v + normals[0] * w
                } else {
                    (tri_vertices.v1() - tri_vertices.v0())
                        .cross(tri_vertices.v2() - tri_vertices.v0())
                        .normalize()
                };
                let intersection = Intersection::new(
                    position,
                    normal.into(),
                    distance,
                    Some(tri_vertices.to_triangle()),
                );
                return Some(intersection);
            }
        }
    }
    None
}

/// Takes a ray and triangle and computes the intersection and normal
#[inline(always)]
pub fn ray_triangle_intersection(
    ray: &Ray3d,
    triangle: &impl TriangleTrait,
    backface_culling: Backfaces,
) -> Option<RayHit> {
    raycast_moller_trumbore(ray, triangle, backface_culling)
}

/// Implementation of the Möller-Trumbore ray-triangle intersection test
pub fn raycast_moller_trumbore(
    ray: &Ray3d,
    triangle: &impl TriangleTrait,
    backface_culling: Backfaces,
) -> Option<RayHit> {
    // Source: https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection
    let vector_v0_to_v1: Vec3A = triangle.v1() - triangle.v0();
    let vector_v0_to_v2: Vec3A = triangle.v2() - triangle.v0();
    let p_vec: Vec3A = ray.direction.cross(vector_v0_to_v2);
    let determinant: f32 = vector_v0_to_v1.dot(p_vec);

    match backface_culling {
        Backfaces::Cull => {
            // if the determinant is negative the triangle is back facing
            // if the determinant is close to 0, the ray misses the triangle
            // This test checks both cases
            if determinant < EPSILON {
                return None;
            }
        }
        Backfaces::Include => {
            // ray and triangle are parallel if det is close to 0
            if determinant.abs() < EPSILON {
                return None;
            }
        }
    }

    let determinant_inverse = 1.0 / determinant;

    let t_vec = ray.origin - triangle.v0();
    let u = t_vec.dot(p_vec) * determinant_inverse;
    if !(0.0..=1.0).contains(&u) {
        return None;
    }

    let q_vec = t_vec.cross(vector_v0_to_v1);
    let v = ray.direction.dot(q_vec) * determinant_inverse;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    // The distance between ray origin and intersection is t.
    let t: f32 = vector_v0_to_v2.dot(q_vec) * determinant_inverse;

    Some(RayHit {
        distance: t,
        uv_coords: (u, v),
    })
}
