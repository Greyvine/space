mod projectile;
mod state;
mod tag;

use crate::{camera::tag::CameraTag, raycast::ray::Ray3d, tag::PlayerModelTag};
use bevy::prelude::*;

use self::{projectile::Projectile, state::DefaultPluginState};

#[derive(Default)]
pub struct ProjectilePlugin;

struct ProjectileTimer(Timer);

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DefaultPluginState>()
            .insert_resource(ProjectileTimer(Timer::from_seconds(0.01, true)))
            .add_system(fire_projectile)
            .add_system(update_bullet);
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
            commands
                .spawn_bundle(PbrBundle {
                    mesh: cube_handle.clone(),
                    material: cube_material_handle.clone(),
                    transform: Transform::from_translation(camera_global_transform.translation)
                        .with_rotation(camera_transform.rotation)
                        .with_scale(Vec3::new(0.1, 0.1, 0.5)),
                    ..Default::default()
                })
                .insert(Name::new("Bullet"))
                .insert(Projectile {
                    direction: ray.direction.into(),
                });
        }
    }
}

fn update_bullet(time: Res<Time>, mut query: Query<(&mut Transform, &Projectile)>) {
    for (mut transform, projectile) in query.iter_mut() {
        transform.translation += 200.0 * projectile.direction * time.delta_seconds();
    }
}
