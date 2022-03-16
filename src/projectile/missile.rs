use bevy::prelude::*;
use lininterp::{InvLerp, Lerp};

use crate::{
    camera::tag::CameraTag,
    raycast::{ray::Ray3d, RayCastMesh, RayCastSource},
    tag::{MyRaycastSet, PlayerModelTag},
};

use super::{
    projectile::Projectile, tag::ProjectileDetectableTag, MissileTimer, Target,
    MAX_DISTANCE_SQUARED,
};

const MISSILE_SPEED: f32 = 0.0;
const MISSILE_TURN_SPEED: f32 = 5.0;
const MAX_MISSILE_TARGETING_DISTANCE_SQUARED: f32 = MAX_DISTANCE_SQUARED / 4.0;

const maxDistancePredict: f32 = 100.0;
const minDistancePredict: f32 = 5.0;
const maxTimePrediction: f32 = 5.0;

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
    source_query: Query<&RayCastSource<MyRaycastSet>>,
    target_query: Query<Entity, With<RayCastMesh<MyRaycastSet>>>,
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

        // let target = source_query
        //     .single()
        //     .intersections
        //     .first()
        //     .and_then(|(x, _)| Some(*x));

        let target = Some(target_query.single());

        let dir: Vec3 = ray.direction.into();

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
                ballistic: true,
                velocity: dir * MISSILE_SPEED,
                direction: dir,
                ray,
                speed: MISSILE_SPEED,
            })
            .insert(Missile { target })
            .insert(ProjectileDetectableTag);
    }
    // }
}

pub(crate) fn update_missile(
    mut commands: Commands,
    time: Res<Time>,
    mut projectile_query: Query<
        (&mut Transform, &mut Projectile, &Missile, Entity),
        Without<PlayerModelTag>,
    >,
    player_query: Query<&GlobalTransform, With<PlayerModelTag>>,
    target_query: Query<(&GlobalTransform, &Target), With<RayCastMesh<MyRaycastSet>>>,
) {
    let player_global_translation = player_query.single().translation;
    for (mut transform, mut projectile, missile, entity) in projectile_query.iter_mut() {
        if (transform.translation - player_global_translation).length_squared()
            > MAX_DISTANCE_SQUARED
        {
            commands.entity(entity).despawn();
        } else {
            if let Some(target) = missile.target {
                if let Ok((target_transform, target)) = target_query.get(target) {
                    // let current_distance = (transform.translation - target_transform.translation).length();
                    // let lead_time_percentage = minDistancePredict.inv_lerp(&maxTimePrediction, &current_distance);
                    // let prediction_time = 0.0.lerp(&maxTimePrediction, lead_time_percentage);
                    // let predicted_translation = target_transform.translation + (target.velocity * prediction_time);

                    // let target_translation = predicted_translation;
                    let target_translation = target_transform.translation;

                    let looking_at_transform = transform.looking_at(target_translation, Vec3::Y);

                    let rotation = looking_at_transform.rotation;

                    transform.rotation = transform
                        .rotation
                        .slerp(rotation, MISSILE_TURN_SPEED * time.delta_seconds());

                    let difference = target_translation - transform.translation;
                    transform.translation +=
                        projectile.speed * difference.normalize() * time.delta_seconds();

                    let acceleration = 400.0;
                    projectile.speed += acceleration * time.delta_seconds();

                    // let difference = target_translation - transform.translation;
                    // let target_direction = difference.normalize();

                    // let acceleration =
                    //     MISSILE_TURN_SPEED * 100.0 * m_direction * time.delta_seconds();
                    // // println!("Velocity {} - Acceleration {}", projectile.velocity, acceleration);

                    // projectile.velocity += acceleration;
                    // transform.translation += projectile.velocity * time.delta_seconds();

                    // transform.translation += projectile.velocity * 5.0 * time.delta_seconds();

                    // transform.translation +=
                    //     MISSILE_SPEED * target_direction * time.delta_seconds();

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
                transform.translation += projectile.velocity * time.delta_seconds();
            }
        }
    }
}
