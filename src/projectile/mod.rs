mod missile;
mod projectile;
mod state;
mod tag;

use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use crate::{
    camera::tag::CameraTag,
    projectile::tag::ProjectileDetectableTag,
    raycast::{
        primitives::{Intersection, IntoUsize, Triangle},
        ray::Ray3d,
        update_raycast::{compute_intersection, triangle_intersection},
        RayCastMesh,
    },
    tag::{MyRaycastSet, PlayerModelTag},
};
use bevy::{
    core::FloatOrd,
    math::Vec3A,
    prelude::*,
    render::mesh::{Indices, VertexAttributeValues},
    tasks::ComputeTaskPool,
};

use self::{
    missile::{fire_missile, update_missile},
    projectile::{Bullet, Projectile},
    state::DefaultPluginState,
};

#[derive(Default)]
pub struct ProjectilePlugin;

struct ProjectileTimer(Timer);
struct MissileTimer(Timer);

#[derive(Component, Default)]
pub struct Target {
    pub velocity: Vec3,
}

const BULLET_SPEED: f32 = 200.0;
const FIRE_RATE: f32 = 0.02;
const MAX_BULLET_DISTANCE: f32 = 1000.0;
pub(crate) const MAX_DISTANCE_SQUARED: f32 = MAX_BULLET_DISTANCE * MAX_BULLET_DISTANCE;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DefaultPluginState>()
            .insert_resource(ProjectileTimer(Timer::from_seconds(FIRE_RATE, true)))
            .insert_resource(MissileTimer(Timer::from_seconds(FIRE_RATE, true)))
            .add_system(fire_projectile)
            .add_system(fire_missile)
            .add_system(update_bullet)
            .add_system(update_missile)
            .add_system(detect_hits);
    }
}

fn fire_projectile(
    mut commands: Commands,
    camera_query: Query<&Transform, With<CameraTag>>,
    player_query: Query<&GlobalTransform, With<PlayerModelTag>>,
    keys: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    mut timer: ResMut<ProjectileTimer>,
) {
    if keys.pressed(KeyCode::LControl) {
        if timer.0.tick(time.delta()).just_finished() {
            let camera_transform = camera_query.single();
            let camera_global_transform = player_query.single();
            let ray = Ray3d::from(camera_transform.compute_matrix());

            let cube_handle = meshes.add(Mesh::from(shape::Cube::default()));
            let cube_material_handle = materials.add(StandardMaterial {
                base_color: Color::WHITE,
                reflectance: 0.02,
                unlit: true,
                ..Default::default()
            });

            let dir = ray.direction.into();
            const BULLET_SPEED: f32 = 200.0;

            commands
                .spawn_bundle(PbrBundle {
                    mesh: cube_handle.clone(),
                    material: cube_material_handle.clone(),
                    transform: Transform::from_translation(camera_global_transform.translation)
                        .with_rotation(camera_transform.rotation)
                        .with_scale(Vec3::new(0.1, 0.1, 0.8)),
                    ..Default::default()
                })
                .insert(Name::new("Bullet"))
                .insert(Bullet)
                .insert(Projectile {
                    ballistic: false,
                    direction: dir,
                    velocity: dir * BULLET_SPEED,
                    ray,
                    speed: BULLET_SPEED,
                })
                .insert(ProjectileDetectableTag);
        }
    }
}

fn update_bullet(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Projectile, Entity), With<Bullet>>,
    player_query: Query<&GlobalTransform, With<PlayerModelTag>>,
) {
    let player_global_translation = player_query.single().translation;
    for (mut transform, projectile, entity) in query.iter_mut() {
        if (transform.translation - player_global_translation).length_squared()
            > MAX_DISTANCE_SQUARED
        {
            commands.entity(entity).despawn();
        } else {
            transform.translation += BULLET_SPEED * projectile.direction * time.delta_seconds();
        }
    }
}

fn detect_hits(
    mut commands: Commands,
    task_pool: Res<ComputeTaskPool>,
    meshes: Res<Assets<Mesh>>,
    culling_query: Query<
        (
            &Visibility,
            Option<&bevy::render::primitives::Aabb>,
            &GlobalTransform,
            Entity,
        ),
        With<RayCastMesh<MyRaycastSet>>,
    >,
    mesh_query: Query<
        (&Handle<Mesh>, &Name, &GlobalTransform, Entity),
        With<RayCastMesh<MyRaycastSet>>,
    >,
    projectiles_query: Query<
        (&Transform, &GlobalTransform, &Projectile, Entity),
        With<ProjectileDetectableTag>,
    >,
) {
    for (transform, projectile_global_transform, projectile, entity) in projectiles_query.iter() {
        let ray = Ray3d::from(transform.compute_matrix());
        let culled_entities: Vec<_> = culling_query
            .iter()
            .filter_map(|(visibility, aab, transform, entity)| {
                if visibility.is_visible {
                    aab.and_then(|x| ray.intersects_aabb(x, &transform.compute_matrix()))
                        .and_then(|[_, far]| if far >= 0.0 { Some(entity) } else { None })
                } else {
                    return None;
                }
            })
            .collect();
        if !culled_entities.is_empty() {
            let picks = Arc::new(Mutex::new(BTreeMap::new()));
            mesh_query.par_for_each(
                &task_pool,
                32,
                |(mesh_handle, name, mesh_global_transform, entity)| {
                    if culled_entities.contains(&entity) {
                        let intersection = meshes.get(mesh_handle).and_then(|x| {
                            compute_bullet_intersection(
                                x,
                                &mesh_global_transform.compute_matrix(),
                                projectile_global_transform,
                                &projectile,
                            )
                        });
                        match intersection {
                            Some(intersection) => {
                                println!("X");
                                picks
                                    .lock()
                                    .unwrap()
                                    .insert(FloatOrd(intersection.distance()), name.as_str());
                            }
                            None => {
                                let distance = (mesh_global_transform.translation
                                    - projectile_global_transform.translation)
                                    .length();
                                if projectile.ballistic && distance < 0.05 {
                                    println!("E");
                                    picks
                                        .lock()
                                        .unwrap()
                                        .insert(FloatOrd(distance), name.as_str());
                                }
                            }
                        }
                    }
                },
            );
            let picks: Vec<_> = Arc::try_unwrap(picks)
                .unwrap()
                .into_inner()
                .unwrap()
                .into_values()
                .collect();
            if !picks.is_empty() {
                if projectile.ballistic {
                    println!("BOOM {:?}", picks);
                    commands.entity(entity).despawn();
                } else {
                    println!("HIT! {:?}", picks);
                }
            }
        } else {
            // commands.entity(entity).remove::<ProjectileDetectableTag>();
        }
    }
}

fn compute_bullet_intersection(
    mesh: &Mesh,
    mesh_to_world: &Mat4,
    projectile_global_transform: &GlobalTransform,
    projectile: &Projectile,
) -> Option<Intersection> {
    if let (
        Some(VertexAttributeValues::Float32x3(positions)),
        Some(VertexAttributeValues::Float32x3(normals)),
        Some(Indices::U32(indices)),
    ) = (
        mesh.attribute(Mesh::ATTRIBUTE_POSITION),
        mesh.attribute(Mesh::ATTRIBUTE_NORMAL),
        mesh.indices(),
    ) {
        compute_bullet_intersection_per_triangle(
            mesh_to_world,
            projectile_global_transform,
            positions,
            normals,
            indices,
            &projectile.ray,
        )
    } else {
        None
    }
}

fn compute_bullet_intersection_per_triangle(
    mesh_to_world: &Mat4,
    projectile_global_transform: &GlobalTransform,
    positions: &Vec<[f32; 3]>,
    normals: &Vec<[f32; 3]>,
    indices: &Vec<u32>,
    ray: &Ray3d,
) -> Option<Intersection> {
    let mut min_pick_distance = 1.0;
    let mut pick_intersection = None;
    let world_to_mesh = mesh_to_world.inverse();

    let projectile_global_translation = projectile_global_transform.translation;

    let mesh_space_ray = Ray3d::new(
        world_to_mesh.transform_point3(projectile_global_translation),
        world_to_mesh.transform_vector3(ray.direction.into()),
    );

    for index in indices.chunks(3) {
        let tri_positions = [
            Vec3A::from(positions[index[0].into_usize()]),
            Vec3A::from(positions[index[1].into_usize()]),
            Vec3A::from(positions[index[2].into_usize()]),
        ];

        let tri_normals = [
            Vec3A::from(normals[index[0].into_usize()]),
            Vec3A::from(normals[index[1].into_usize()]),
            Vec3A::from(normals[index[2].into_usize()]),
        ];

        if let Some(i) = triangle_intersection(
            tri_positions,
            Some(tri_normals),
            min_pick_distance,
            mesh_space_ray,
        ) {
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

    pick_intersection
}
