use bevy::prelude::*;

use crate::{
    camera::tag::CameraTag,
    raycast::{ray::Ray3d, RayCastMesh, RayCastSource},
    tag::{MyRaycastSet, PlayerModelTag},
};

use super::{
    projectile::Projectile, tag::ProjectileDetectableTag, MissileTimer, MAX_DISTANCE_SQUARED,
};

const MISSILE_SPEED: f32 = 200.0;
const MISSILE_TURN_SPEED: f32 = 10.0;
const MAX_MISSILE_TARGETING_DISTANCE_SQUARED: f32 = MAX_DISTANCE_SQUARED / 4.0;

#[derive(Component)]
pub(crate) struct Missile {
    pub target: Option<Entity>,
}

pub(crate) fn fire_missile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<CameraTag>>,
    player_query: Query<&GlobalTransform, With<PlayerModelTag>>,
    keys: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    source_query: Query<&mut RayCastSource<MyRaycastSet>>,
    // time: Res<Time>,
    // mut timer: ResMut<MissileTimer>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let spaceship_handle = asset_server.load("models/spaceship.gltf#Mesh0/Primitive0");

        // if timer.0.tick(time.delta()).just_finished() {
        let camera_transform = camera_query.single();
        let camera_global_transform = player_query.single();
        let ray = Ray3d::from(camera_transform.compute_matrix());

        let cube_material_handle = materials.add(StandardMaterial {
            base_color: Color::MIDNIGHT_BLUE,
            reflectance: 0.5,
            unlit: false,
            ..Default::default()
        });

        let target = source_query
            .single()
            .intersections
            .first()
            .and_then(|(x, _)| Some(*x));

        commands
            .spawn_bundle(PbrBundle {
                mesh: spaceship_handle.clone(),
                material: cube_material_handle.clone(),
                transform: Transform::from_translation(camera_global_transform.translation)
                    .with_rotation(camera_transform.rotation)
                    .with_scale(Vec3::new(1.4, 1.4, 5.0)),
                ..Default::default()
            })
            .insert(Name::new("Missile"))
            .insert(Projectile {
                direction: ray.direction.into(),
                ray,
            })
            .insert(Missile { target })
            .insert(ProjectileDetectableTag);
    }
    // }
}

pub(crate) fn update_missile(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Projectile, &Missile, Entity)>,
    player_query: Query<&GlobalTransform, With<PlayerModelTag>>,
    target_query: Query<&GlobalTransform, With<RayCastMesh<MyRaycastSet>>>,
) {
    let player_global_translation = player_query.single().translation;
    for (mut transform, projectile, missile, entity) in query.iter_mut() {
        if (transform.translation - player_global_translation).length_squared()
            > MAX_DISTANCE_SQUARED
        {
            commands.entity(entity).despawn();
        } else {
            if let Some(target) = missile.target {
                if let Ok(target_transform) = target_query.get(target) {
                    let rotation = transform
                        .looking_at(target_transform.translation, Vec3::Y)
                        .rotation;
                    transform.rotation = transform
                        .rotation
                        .slerp(rotation, 10.0 * time.delta_seconds());

                    let difference = target_transform.translation - transform.translation;
                    let target_direction = difference.normalize();
                    transform.translation +=
                        MISSILE_SPEED * target_direction * time.delta_seconds();

                    // let difference = target_transform.translation - transform.translation;
                    // let target_direction = difference.normalize();
                    // transform.translation += MISSILE_TURN_SPEED * target_direction * time.delta_seconds();

                    // let target_distance = difference.length_squared();
                    // // TODO Make missile stop targeting when angle between target and initial direction is too extreme
                    // if target_distance < MAX_MISSILE_TARGETING_DISTANCE_SQUARED {
                    //     let target_direction = difference.normalize();
                    //     transform.translation += MISSILE_TURN_SPEED * target_direction * time.delta_seconds();
                    // }
                }
            } else {
                transform.translation +=
                    MISSILE_SPEED * projectile.direction * time.delta_seconds();
            }
        }
    }
}
