use bevy::{
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
    render::{options::WgpuOptions, render_resource::WgpuFeatures},
};

use space::camera::tag::*;
use space::scale::*;
use space::{
    camera::*,
    controller::{tag::ControllerPlayerTag, ControllerPlugin},
};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Planet;

fn main() {
    App::new()
        .insert_resource(WgpuOptions {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(ControllerPlugin)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.1,
        })
        .add_startup_system(setup.system())
        .add_startup_system(spawn_marker.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dimensions = Vec3::new(1.2, 1.0, 5.0) * M_TO_UNIT_SCALE;

    let cube_handle = meshes.add(Mesh::from(shape::Cube::default()));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        reflectance: 0.02,
        unlit: false,
        ..Default::default()
    });

    let body = commands
        .spawn_bundle((GlobalTransform::identity(), Transform::identity()))
        .insert(ControllerPlayerTag)
        .id();

    let player = commands
        .spawn_bundle(PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_scale(dimensions),
            ..Default::default()
        })
        .insert(Wireframe)
        .id();

    let camera = commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 2.0, 15.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(PlayerTag)
        .id();

    commands.entity(body).push_children(&[player, camera]);
}

fn spawn_marker(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_handle = meshes.add(Mesh::from(shape::Cube::default()));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::RED,
        reflectance: 0.02,
        unlit: false,
        ..Default::default()
    });

    commands
        .spawn_bundle(PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_translation(Vec3::Z * -15.0),
            ..Default::default()
        })
        .insert(Wireframe);
}
