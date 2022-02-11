use bevy::{pbr::wireframe::Wireframe, prelude::*};

use crate::{
    material::{CustomMaterial, SkyboxTextureConversion},
    mesh::QuadSphere,
    origin::SimulationBundle,
    scale::KM_TO_UNIT_SCALE,
    tag::NonPlayerTag,
};

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

    commands
        .spawn_bundle(PbrBundle {
            mesh: sphere_handle,
            material: material_handle,
            transform: Transform::from_translation(Vec3::Z * radius * 2.0),
            ..Default::default()
        })
        .insert(NonPlayerTag)
        .insert_bundle(SimulationBundle::new(Vec3::Z * radius * 2.0));
}

#[derive(Component)]
pub struct EarthTag;

pub fn spawn_earth(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut skybox_conversion: ResMut<SkyboxTextureConversion>,
) {
    let radius = 6_378.0 * KM_TO_UNIT_SCALE;
    let texture_handle = asset_server.load("textures/earth-cube-map.png");
    let height_map_handle = asset_server.load("textures/height-cube-map-low.png");
    skybox_conversion.make_array(texture_handle.clone());
    skybox_conversion.make_array(height_map_handle.clone());

    let material_handle = materials.add(CustomMaterial {
        base_color_texture: Some(texture_handle.clone()),
        height_map_texture: Some(height_map_handle.clone()),
        color: Color::GREEN,
    });

    let sphere_handle = meshes.add(Mesh::from(QuadSphere {
        radius,
        subdivisions: 30,
    }));

    commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: sphere_handle,
            material: material_handle,
            ..Default::default()
        })
        // .insert(Wireframe)
        .insert(EarthTag)
        .insert(NonPlayerTag)
        .insert_bundle(SimulationBundle::new(Vec3::new(
            radius * 2.0,
            0.0,
            -radius * 10.0,
        )));
}
