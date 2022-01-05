use bevy::prelude::*;

use crate::scale::KM_TO_UNIT_SCALE;

pub fn spawn_moon(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 1_737.4 * KM_TO_UNIT_SCALE;
    let texture_handle = asset_server.load("textures/moon.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    });

    let sphere_handle = meshes.add(Mesh::from(shape::Icosphere {
        radius,
        subdivisions: 10,
    }));

    commands.spawn_bundle(PbrBundle {
        mesh: sphere_handle,
        material: material_handle,
        transform: Transform::from_translation(Vec3::Z * radius * 2.0),
        ..Default::default()
    });
}

#[derive(Component)]
pub struct EarthTag;

pub fn spawn_earth(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 6_378.0 * KM_TO_UNIT_SCALE;
    let texture_handle = asset_server.load("textures/earth.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    });

    let sphere_handle = meshes.add(Mesh::from(shape::Icosphere {
        radius,
        subdivisions: 10,
    }));

    commands.spawn_bundle(PbrBundle {
        mesh: sphere_handle,
        material: material_handle,
        transform: Transform::from_translation(Vec3::new(radius * 2.0, 0.0, -radius * 10.0)),
        ..Default::default()
    }).insert(EarthTag);
}
