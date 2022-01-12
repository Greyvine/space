use bevy::{
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
    render::{options::WgpuOptions, render_resource::WgpuFeatures},
};

use space::{
    camera::tag::*,
    origin::{OriginRebasingPlugin, SimulationBundle},
    planet::spawn_moon,
    tag::PlayerTag, lod::{lod_system, LodBundle, LodTag},
};
use space::{
    camera::*,
    controller::{tag::ControllerPlayerTag, ControllerPlugin},
};
use space::{
    planet::{spawn_earth, EarthTag},
    scale::*,
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
        .add_plugin(OriginRebasingPlugin)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.5,
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup.system())
        .add_startup_system(spawn_monkey.system())
        .add_system(lod_system.system())
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
        .insert(PlayerTag)
        .insert_bundle(SimulationBundle::default())
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
            perspective_projection: PerspectiveProjection {
                far: 10.0 * AU_TO_UNIT_SCALE,
                // near: 0.5 * AU_TO_UNIT_SCALE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CameraTag)
        .id();

    commands.entity(body).push_children(&[player, camera]);
}

fn spawn_monkey(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    asset_server.load_folder("models/monkey").unwrap();

    // Then any asset in the folder can be accessed like this:
    let monkey_handle = asset_server.get_handle("models/monkey/Monkey-low.gltf#Mesh0/Primitive0");

    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });

    // monkey
    commands
        .spawn_bundle(PbrBundle {
            mesh: monkey_handle,
            material: material_handle.clone(),
            transform: Transform::from_xyz(-20.0, 0.0, -10.0),
            ..Default::default()
        })
        .insert_bundle(SimulationBundle::from_transform(Transform::from_scale(
            Vec3::splat(3.0),
        )))
        .insert(LodBundle::new(
            "models/monkey/Monkey.gltf#Mesh0/Primitive0".to_string(),
            "models/monkey/Monkey-medium.gltf#Mesh0/Primitive0".to_string(),
            "models/monkey/Monkey-low.gltf#Mesh0/Primitive0".to_string(),
        ))
        .insert(Wireframe)
        .insert(LodTag);
}
